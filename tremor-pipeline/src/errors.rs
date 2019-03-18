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

#![allow(deprecated)]

use crate::ValueType;
use error_chain::*;
use serde_json;
use serde_yaml;
use std;

error_chain! {
    links {
        Script(tremor_script::errors::Error, tremor_script::errors::ErrorKind);
    }
    foreign_links {
        YAMLError(serde_yaml::Error) #[doc = "Error during yalm parsing"];
        JSONError(serde_json::Error);
        Io(std::io::Error) #[cfg(unix)];
        FromUTF8Error(std::string::FromUtf8Error);
        UTF8Error(std::str::Utf8Error);
        ParseIntError(std::num::ParseIntError);
        ParseFloatError(std::num::ParseFloatError);

    }

    errors {
        CyclicGraphError(g: String) {
            description("Cycle detected in graph")
                display("Cycle detected in graph: {}", g)
        }
        MissingOpConfig(e: String) {
            description("Operator config is missing")
                display("Operator config for {} is missing", e)
        }
        ExtraOpConfig(e: String) {
            description("Operator has extra config")
                display("Operator {} has a config but can't be configured", e)
        }
        TypeError(l: String, expected: ValueType, got: ValueType) {
            description("Type error")
                display("expected type '{}' but found type '{}' in {}", expected, got, l)
        }

        UnknownOp(n: String, o: String) {
            description("Unknown operator")
                display("Unknown operator: {}::{}", n, o)
        }

        MissingSteps(t: String) {
            description("missing steps")
                display("missing steps in: '{}'", t)
        }

        BadOpConfig(e: String) {
            description("Operator config has a bad syntax")
                display("Operator config has a bad syntax: {}", e)
        }

        UnknownNamespace(n: String) {
            description("Unknown namespace")
                display("Unknown namespace: {}", n)
        }


        UnknownSubPipeline(p: String) {
            description("Reference to unknown sub-pipeline")
                display("The sub-pipelines '{}' is not defined", p)
        }

        UnknownPipeline(p: String) {
            description("Reference to unknown pipeline")
                display("The pipelines '{}' is not defined", p)
        }

        PipelineStartError(p: String) {
            description("Failed to start pipeline")
                display("Failed to start pipeline '{}'", p)
        }

        OnrampError(i: u64) {
            description("Error in onramp")
                display("Error in onramp '{}'", i)
        }

        OnrampMissingPipeline(o: String) {
            description("Onramp is missing a pipeline")
                display("The onramp '{}' is missing a pipeline", o)
        }
        InvalidInfluxData(s: String) {
            description("Invalid Influx Line Protocol data")
                display("Invalid Influx Line Protocol data: {}", s)
        }
        InvalidJsonData(s: String) {
            description("Invalid JSON")
                display("Invalid JSON data: {}", s)
        }

        BadOutputid(i: usize) {
            description("Bad output pipeline id.")
                display("Bad output pipeline id {}", i - 1)
        }
    }
}
