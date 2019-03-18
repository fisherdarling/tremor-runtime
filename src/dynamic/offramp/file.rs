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

//! # File Offramp
//!
//! Writes events to a file, one event per line
//!
//! ## Configuration
//!
//! See [Config](struct.Config.html) for details.

use super::{Offramp, OfframpImpl};
use crate::dynamic::codec::Codec;
use crate::dynamic::{Event, OpConfig};
use crate::errors::*;
use crate::system::PipelineAddr;
use crate::url::TremorURL;
use hashbrown::HashMap;
use serde_yaml;
use std::fs::File as FSFile;
use std::io::Write;
use tremor_pipeline::EventValue;

/// An offramp that write a given file
#[derive(Debug)]
pub struct File {
    file: FSFile,
    pipelines: HashMap<TremorURL, PipelineAddr>,
}

#[derive(Deserialize)]
pub struct Config {
    /// Filename to write to
    pub file: String,
}

impl OfframpImpl for File {
    fn from_config(config: &Option<OpConfig>) -> Result<Box<dyn Offramp>> {
        if let Some(config) = config {
            let config: Config = serde_yaml::from_value(config.clone())?;
            let file = FSFile::create(config.file)?;
            Ok(Box::new(File {
                file,
                pipelines: HashMap::new(),
            }))
        } else {
            Err("Blackhole offramp requires a config".into())
        }
    }
}

impl Offramp for File {
    // TODO
    fn on_event(&mut self, codec: &Box<dyn Codec>, _input: String, event: Event) {
        for event in event.into_iter() {
            if let Ok(EventValue::Raw(ref raw)) = codec.encode(event.value) {
                //TODO: Error handling
                self.file.write_all(&raw).unwrap();
                self.file.write_all(b"\n").unwrap();
            }
        }
    }
    fn add_pipeline(&mut self, id: TremorURL, addr: PipelineAddr) {
        self.pipelines.insert(id, addr);
    }
    fn remove_pipeline(&mut self, id: TremorURL) -> bool {
        self.pipelines.remove(&id);
        self.pipelines.is_empty()
    }
    fn default_codec(&self) -> &str {
        "json"
    }
}
