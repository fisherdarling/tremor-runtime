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

// Screw actix web, it's not our fault!
#![allow(clippy::type_complexity)]

use crate::api::{content_type, reply, ResourceType, State};
use actix_web::http::StatusCode;
use actix_web::{error, HttpRequest, HttpResponse, Path, Responder};
use hashbrown::HashMap;
use tremor_runtime::errors::*;
use tremor_runtime::url::TremorURL;

#[derive(Serialize)]
struct BindingWrap {
    artefact: tremor_runtime::config::Binding,
    instances: Vec<String>,
}

pub fn list_artefact(req: HttpRequest<State>) -> impl Responder {
    let res = req.state().world.repo.list_bindings();
    reply(req, res, 200)
}

pub fn publish_artefact((req, data_raw): (HttpRequest<State>, String)) -> impl Responder {
    let data: tremor_runtime::config::Binding = match content_type(&req) {
        Some(ResourceType::Yaml) => serde_yaml::from_str(&data_raw).unwrap(),
        Some(ResourceType::Json) => serde_json::from_str(&data_raw).unwrap(),
        None => return HttpResponse::InternalServerError().finish(),
    };
    let url = TremorURL::parse(&format!("/binding/{}", data.id))
        .map_err(|_e| error::ErrorBadRequest("bad url"))
        .unwrap();
    let res = req.state().world.repo.publish_binding(&url, data);
    reply(req, res, 201)
}

pub fn unpublish_artefact((req, path): (HttpRequest<State>, Path<(String)>)) -> impl Responder {
    let url = TremorURL::parse(&format!("/binding/{}", path))
        .map_err(|e| error::ErrorBadRequest(format!("bad url: {}", e)))
        .unwrap();
    let res = req.state().world.repo.unpublish_binding(&url);
    reply(req, res, 200)
}

pub fn get_artefact((req, id): (HttpRequest<State>, Path<String>)) -> impl Responder {
    let url = TremorURL::parse(&format!("/binding/{}", id))
        .map_err(|_e| error::ErrorBadRequest("bad url"))
        .unwrap();

    let res = req
        .state()
        .world
        .repo
        .find_binding(&url)
        .map_err(|_e| error::ErrorInternalServerError("lookup failed"));

    match res {
        Ok(res) => match res {
            Some(res) => {
                let res: Result<BindingWrap> = Ok(BindingWrap {
                    artefact: res.artefact,
                    instances: res.instances,
                });
                reply(req, res, 200)
            }
            None => HttpResponse::build(StatusCode::from_u16(404).unwrap()).finish(),
        },
        Err(_) => HttpResponse::build(StatusCode::from_u16(404).unwrap()).finish(),
    }
}

pub fn get_servant((req, path): (HttpRequest<State>, Path<(String, String)>)) -> impl Responder {
    let url = TremorURL::parse(&format!("/binding/{}/{}", path.0, path.1))
        .map_err(|_e| error::ErrorBadRequest("bad url"))
        .unwrap();
    let res = req.state().world.reg.find_binding(&url);
    reply(req, res, 200)
}

// We really don't want to deal with that!
#[allow(clippy::implicit_hasher)]
pub fn link_servant(
    (req, path, data_raw): (HttpRequest<State>, Path<(String, String)>, String),
) -> impl Responder {
    let data: HashMap<String, String> = match content_type(&req) {
        Some(ResourceType::Yaml) => serde_yaml::from_str(&data_raw).unwrap(),
        Some(ResourceType::Json) => serde_json::from_str(&data_raw).unwrap(),
        None => return HttpResponse::InternalServerError().finish(),
    };
    let url = TremorURL::parse(&format!("/binding/{}/{}", path.0, path.1))
        .map_err(|_e| error::ErrorBadRequest("bad url"))
        .unwrap();
    let res = req.state().world.link_binding(&url, data);
    reply(req, res, 201)
}

// We really don't want to deal with that!
#[allow(clippy::implicit_hasher)]
pub fn unlink_servant((req, path): (HttpRequest<State>, Path<(String, String)>)) -> impl Responder {
    /*let data: HashMap<String, String> = match content_type(&req) {
        Some(ResourceType::Yaml) => serde_yaml::from_slice(&data_raw).unwrap(),
        Some(ResourceType::Json) => serde_json::from_slice(&data_raw).unwrap(),
        None => return HttpResponse::InternalServerError().finish(),
    };
     */
    let url = TremorURL::parse(&format!("/binding/{}/{}", path.0, path.1))
        .map_err(|_e| error::ErrorBadRequest("bad url"))
        .unwrap();
    let res = req.state().world.unlink_binding(&url, HashMap::new());
    reply(req, res, 200)
}
