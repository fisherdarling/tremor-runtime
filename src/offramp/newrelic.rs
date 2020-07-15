// Copyright 2018-2020, Wayfair GmbH
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

//! # `NewRelic` Offramp
//!
//! Buffers and send messages to `NewRelic`
//!
//! ## Configuration
//!
//! See [Config](struct.Config.html) for details.
use std::io::{Cursor, Write};

use async_std::task::block_on;
use chrono::prelude::Utc;
use hashbrown::HashMap;
use http_types::headers::CONTENT_TYPE;
use libflate::{finish, gzip};
use log::debug;
use simd_json::BorrowedValue;

use crate::offramp::prelude::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// NewRelic license/insert_key key
    #[serde(flatten)]
    pub key: Key,
    /// Choose if logs should be compressed before sending to newrelic
    /// This avoids extra egress costs but increases CPU usage on tremor server
    #[serde(default)]
    pub compress_logs: bool,
    /// Region to use to send logs
    /// use Europe in case you need to be GDPR compliant
    #[serde(default)]
    pub region: Region,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Key {
    /// NewRelic license key
    LicenseKey(String),
    /// NewRelic insert only key
    InsertKey(String),
}

impl ConfigImpl for Config {}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Region {
    Usa,
    Europe,
    #[cfg(test)]
    Test,
}

impl Default for Region {
    fn default() -> Self {
        Self::Usa
    }
}

impl Region {
    pub fn logs_url(&self) -> &str {
        match self {
            Self::Usa => "https://log-api.newrelic.com/log/v1",
            Self::Europe => "https://log-api.eu.newrelic.com/log/v1",
            #[cfg(test)]
            Self::Test => "http://localhost:23456/newrelic",
        }
    }
}

pub struct NewRelic {
    config: Config,
    pipelines: HashMap<TremorURL, pipeline::Addr>,
    postprocessors: Postprocessors,
}

impl offramp::Impl for NewRelic {
    fn from_config(config: &Option<OpConfig>) -> Result<Box<dyn Offramp>> {
        if let Some(config) = config {
            let config = Config::new(config)?;
            Ok(Box::new(NewRelic {
                config,
                pipelines: HashMap::new(),
                postprocessors: vec![],
            }))
        } else {
            Err("Missing config for newrelic offramp".into())
        }
    }
}

impl Offramp for NewRelic {
    fn start(&mut self, _codec: &Box<dyn Codec>, postprocessors: &[String]) -> Result<()> {
        self.postprocessors = make_postprocessors(postprocessors)?;
        Ok(())
    }

    fn on_event(&mut self, _codec: &Box<dyn Codec>, _input: String, event: Event) -> Result<()> {
        // TODO: Document this, if one of the log entries cannot be decoded, the whole batch will be lost because
        // of the collect::<Result<_>>
        let payload = NewRelicPayload {
            logs: event
                .value_iter()
                .map(Self::value_to_newrelic_log)
                .collect::<Result<_>>()?,
        };

        debug!("Sending a batch of {} items", payload.logs.len());

        block_on(self.send(&payload))?;
        Ok(())
    }

    fn default_codec(&self) -> &str {
        "json"
    }

    fn add_pipeline(&mut self, id: TremorURL, addr: pipeline::Addr) {
        self.pipelines.insert(id, addr);
    }

    fn remove_pipeline(&mut self, id: TremorURL) -> bool {
        self.pipelines.remove(&id);
        self.pipelines.is_empty()
    }
}

impl NewRelic {
    async fn send(&mut self, newrelic_payload: &NewRelicPayload) -> Result<()> {
        let (key, value) = self.auth_headers();
        let mut buffer = Vec::with_capacity(10240);
        {
            let mut writer = self.get_writer(&mut buffer)?;
            simd_json::to_writer::<NewRelicPayload, _>(&mut writer, newrelic_payload)?;
        }

        debug!("sending {} bytes", buffer.len());

        let request = surf::post(self.config.region.logs_url())
            .set_header(key, value)
            .body_bytes(buffer)
            .set_header(CONTENT_TYPE, self.content_type());

        let mut response = request.await?;

        if !response.status().is_success() {
            let body = match response.body_string().await {
                Ok(body) => body,
                Err(err) => format!("failed to load body {}", err),
            };
            return Err(format!(
                "error sending newrelic logs\nresponse: {:?}\nreturned body: {}",
                response, body
            )
            .into());
        }

        if log::log_enabled!(log::Level::Debug) {
            let body = match response.body_string().await {
                Ok(body) => body,
                Err(err) => format!("failed to load body {}", err),
            };
            debug!("newrelic response: {:?}\nbody: {}", response, body);
        }

        Ok(())
    }

    fn auth_headers(&self) -> (&str, &str) {
        match self.config.key {
            Key::LicenseKey(ref key) => ("X-License-Key", key),
            Key::InsertKey(ref key) => ("X-Insert-Key", key),
        }
    }

    fn get_writer<'a>(&self, buffer: &'a mut Vec<u8>) -> Result<Box<dyn Write + 'a>> {
        if self.config.compress_logs {
            // Just unwrap as we are writing to a Vec
            trace!("using gzip writer");
            Ok(Box::new(finish::AutoFinishUnchecked::new(
                gzip::Encoder::new(buffer)?,
            )))
        } else {
            trace!("using plain text writer");
            Ok(Box::new(Cursor::new(buffer)))
        }
    }

    fn content_type(&self) -> &str {
        if self.config.compress_logs {
            "application/gzip"
        } else {
            "application/json"
        }
    }

    fn value_to_newrelic_log(value: &BorrowedValue<'_>) -> Result<NewRelicLog> {
        let timestamp = value
            .get("timestamp")
            .and_then(BorrowedValue::as_i64)
            .unwrap_or_else(|| Utc::now().timestamp_millis());

        Ok(NewRelicLog {
            timestamp,
            message: simd_json::to_string::<BorrowedValue<'_>>(value)?,
        })
    }
}

#[derive(Debug, Serialize)]
struct NewRelicPayload {
    logs: Vec<NewRelicLog>,
}

#[derive(Debug, Serialize)]
struct NewRelicLog {
    timestamp: i64,
    message: String,
}

#[cfg(test)]
mod tests {
    use super::{Config, Key, Region};

    #[test]
    fn config_shape() {
        let config = Config {
            key: Key::LicenseKey("asdas".into()),
            compress_logs: true,
            region: Default::default(),
        };
        let config_output = simd_json::to_string::<Config>(&config).expect("failed to dump config");
        assert_eq!(
            config_output,
            r#"{"license_key":"asdas","compress_logs":true,"region":"usa"}"#
        );
    }

    #[test]
    fn region_url() {
        assert_eq!(
            Region::default().logs_url(),
            "https://log-api.newrelic.com/log/v1"
        );
        assert_eq!(
            Region::Usa.logs_url(),
            "https://log-api.newrelic.com/log/v1"
        );
        assert_eq!(
            Region::Europe.logs_url(),
            "https://log-api.eu.newrelic.com/log/v1"
        );
    }
}