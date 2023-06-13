use std::{collections::HashMap, u8};

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    enums::{headers::ContentType, method::Method},
    http::http_header_mapper::HttpHeaderMapper,
};

use super::header::Header;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Request {
    id: String,
    name: String,
    pub uri: String,
    pub headers: HashMap<String, Header>,
    pub method: Method,
    pub body: Vec<u8>,
}

impl Request {
    pub fn new(method: Method, uri: String) -> Request {
        Request {
            id: Uuid::new_v4().to_string(),
            name: String::new(),
            uri,
            method,
            headers: HashMap::new(),
            body: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn add_body(&mut self, body: &str) -> &mut Request {
        let (header_name, header) = HttpHeaderMapper::content_type(ContentType::Text);

        self.body = body.as_bytes().to_vec();
        self.add_header(&header_name, header);

        self
    }

    pub fn add_body_binary(&mut self, body: &Vec<u8>) -> &mut Request {
        self.body = body.clone();

        self
    }

    pub fn add_header(&mut self, name: &str, value: Header) -> &mut Request {
        self.headers.entry(name.to_owned()).or_insert(value);

        self
    }
}
