use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::models::{header::Header, request::Request};

use super::{json_header::JsonHeader, json_method::JsonMethod};

#[derive(Serialize, Deserialize)]
pub struct JsonRequest {
    pub uri: String,
    pub headers: HashMap<String, JsonHeader>,
    pub method: JsonMethod,
    pub body: Vec<u8>,
}

impl JsonRequest {
    pub fn new(request: &Request) -> JsonRequest {
        JsonRequest {
            uri: request.uri.clone(),
            method: JsonMethod::new(&request.method),
            body: request.body.clone(),
            headers: JsonRequest::to_json_headers(&request.headers),
        }
    }

    fn to_json_headers(headers: &HashMap<String, Header>) -> HashMap<String, JsonHeader> {
        let mut json_headers: HashMap<String, JsonHeader> = HashMap::new();

        for (k, v) in headers {
            json_headers.insert(k.to_string(), JsonHeader::new(v));
        }

        json_headers
    }
}
