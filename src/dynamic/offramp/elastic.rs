// Copyright 2018-2019, Wayfair GmbH
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! # Elastic Search Offramp
//!
//! The Elastic Search Offramp uses batch writes to send data to elastic search
//!
//! ## Configuration
//!
//! See [Config](struct.Config.html) for details.
//!
//! ## Input Variables
//!   * `index` - index to write to (required)
//!   * `doc-type` - document type for the event (required)
//!   * `pipeline` - pipeline to use
//!
//! ## Outputs
//!
//! The 1st additional output is used to send divert messages that can not be
//! enqueued due to overload

use super::{Offramp, OfframpImpl};
use crate::async_sink::{AsyncSink, SinkDequeueError};
use crate::dflt;
use crate::dynamic::codec::Codec;
use crate::dynamic::{Event, EventValue, OpConfig};
use crate::errors::*;
use crate::system::{PipelineAddr, PipelineMsg};
use crate::url::TremorURL;
use crate::utils::{duration_to_millis, nanotime};
use elastic::client::prelude::BulkErrorsResponse;
use elastic::client::requests::BulkRequest;
use elastic::client::{Client, SyncSender};
use elastic::prelude::SyncClientBuilder;
use hashbrown::HashMap;
use hostname::get_hostname;
use serde_json::{json, Value};
use serde_yaml;
use std::convert::From;
use std::sync::mpsc::channel;
use std::time::Instant;
use std::{fmt, str};
use threadpool::ThreadPool;
use tremor_pipeline::MetaMap;

#[derive(Debug, Deserialize)]
pub struct Config {
    /// list of endpoint urls
    pub endpoints: Vec<String>,
    /// maximum number of paralel in flight batches (default: 4)
    #[serde(default = "dflt::d_4")]
    pub concurrency: usize,
}

#[derive(Clone)]
struct Destination {
    client: Client<SyncSender>,
    url: String,
}

impl fmt::Debug for Destination {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.url)
    }
}

#[derive(Debug)]
pub struct Elastic {
    client_idx: usize,
    clients: Vec<Destination>,
    config: Config,
    pool: ThreadPool,
    queue: AsyncSink<u64>,
    hostname: String,
    pipelines: HashMap<TremorURL, PipelineAddr>,
}

impl OfframpImpl for Elastic {
    fn from_config(config: &Option<OpConfig>) -> Result<Box<dyn Offramp>> {
        if let Some(config) = config {
            let config: Config = serde_yaml::from_value(config.clone())?;
            let clients: Vec<Destination> = config
                .endpoints
                .iter()
                .map(|s| Destination {
                    client: SyncClientBuilder::new()
                        .base_url(s.clone())
                        .build()
                        .unwrap(),
                    url: s.clone(),
                })
                .collect();

            let pool = ThreadPool::new(config.concurrency);
            let queue = AsyncSink::new(config.concurrency);
            let hostname = match get_hostname() {
                Some(h) => h,
                None => "tremor-host.local".to_string(),
            };

            Ok(Box::new(Elastic {
                client_idx: 0,
                pipelines: HashMap::new(),
                config,
                pool,
                clients,
                queue,
                hostname,
            }))
        } else {
            Err("Elastic offramp requires a configuration.".into())
        }
    }
}

impl Elastic {
    fn flush(client: &Client<SyncSender>, payload: &str) -> Result<u64> {
        let start = Instant::now();
        let req = BulkRequest::new(payload.to_owned());
        let res = client.request(req).send()?;
        for item in res.into_response::<BulkErrorsResponse>()? {
            error!("Elastic Search item error: {:?}", item);
        }
        let d = start.elapsed();
        let d = duration_to_millis(d);
        Ok(d)
    }

    fn enqueue_send_future(&mut self, payload: String) -> Result<()> {
        self.client_idx = (self.client_idx + 1) % self.clients.len();
        let destination = self.clients[self.client_idx].clone();
        let (tx, rx) = channel();
        let pipelines: Vec<(TremorURL, PipelineAddr)> = self
            .pipelines
            .iter()
            .map(|(i, p)| (i.clone(), p.clone()))
            .collect();
        self.pool.execute(move || {
            let r = Self::flush(&destination.client, payload.as_str());
            let mut m = MetaMap::new();
            if let Ok(t) = r {
                m.insert("time".to_string(), json!(t));
            } else {
                error!("Elastic search error: {:?}", r);
                m.insert("error".to_string(), json!("Failed to send to ES"));
            };
            let insight = Event {
                is_batch: false,
                id: 0,
                meta: m,
                value: tremor_pipeline::EventValue::None,
                ingest_ns: nanotime(),
                kind: None,
            };

            for (pid, p) in pipelines {
                if p.addr.send(PipelineMsg::Insight(insight.clone())).is_err() {
                    error!("Failed to send contraflow to pipeline {}", pid)
                };
            }

            // TODO: Handle contraflow for notification
            let _ = tx.send(r);
        });
        self.queue.enqueue(rx)?;
        Ok(())
    }
    fn maybe_enque(&mut self, payload: String) -> Result<()> {
        match self.queue.dequeue() {
            Err(SinkDequeueError::NotReady) if !self.queue.has_capacity() => {
                //TODO: how do we handle this?
                error!("Dropped data due to es overload");
                Err("Dropped data due to es overload".into())
            }
            _ => {
                if self.enqueue_send_future(payload).is_err() {
                    // TODO: handle reply to the pipeline
                    error!("Failed to enqueue send request to elastic");
                    Err("Failed to enqueue send request to elastic".into())
                } else {
                    Ok(())
                }
            }
        }
    }
}

impl Offramp for Elastic {
    // We enforce json here!
    fn on_event(&mut self, _codec: &Box<dyn Codec>, _input: String, event: Event) {
        let mut payload = String::from("");

        for event in event.into_iter() {
            let index = if let Some(Value::String(index)) = event.meta.get("index") {
                index
            } else {
                error!("'index' not set for elastic offramp!");
                return;
            };
            let doc_type = if let Some(Value::String(doc_type)) = event.meta.get("doc_type") {
                doc_type
            } else {
                error!("'doc-type' not set for elastic offramp!");
                return;
            };
            let pipeline = if let Some(Value::String(pipeline)) = event.meta.get("pipeline") {
                Some(pipeline)
            } else {
                None
            };
            match pipeline {
                None => payload.push_str(
                    json!({
                    "index":
                    {
                        "_index": index,
                        "_type": doc_type
                    }})
                    .to_string()
                    .as_str(),
                ),
                Some(ref pipeline) => payload.push_str(
                    json!({
                    "index":
                    {
                        "_index": index,
                        "_type": doc_type,
                        "pipeline": pipeline
                    }})
                    .to_string()
                    .as_str(),
                ),
            };
            payload.push('\n');
            match event.value {
                EventValue::JSON(json) => {
                    payload.push_str(serde_json::to_string(&json).unwrap().as_str());
                    payload.push('\n');
                }
                _ => error!("Event data needs to be json"),
            }
        }
        let _ = self.maybe_enque(payload);
    }
    fn default_codec(&self) -> &str {
        "pass"
    }
    fn add_pipeline(&mut self, id: TremorURL, addr: PipelineAddr) {
        self.pipelines.insert(id, addr);
    }
    fn remove_pipeline(&mut self, id: TremorURL) -> bool {
        self.pipelines.remove(&id);
        self.pipelines.is_empty()
    }
}
