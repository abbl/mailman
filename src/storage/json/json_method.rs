use serde::{Deserialize, Serialize};

use crate::enums::method::Method;

#[derive(Serialize, Deserialize)]
pub enum JsonMethod {
    Get,
    Post,
    Patch,
    Put,
    Update,
    Delete,
}

impl JsonMethod {
    pub fn new(method: &Method) -> JsonMethod {
        match method {
            Method::Get => JsonMethod::Get,
            Method::Post => JsonMethod::Post,
            Method::Patch => JsonMethod::Patch,
            Method::Put => JsonMethod::Put,
            Method::Update => JsonMethod::Update,
            Method::Delete => JsonMethod::Delete,
        }
    }

    pub fn to_method(json_method: &JsonMethod) -> Method {
        match json_method {
            JsonMethod::Get => Method::Get,
            JsonMethod::Post => Method::Post,
            JsonMethod::Patch => Method::Patch,
            JsonMethod::Put => Method::Put,
            JsonMethod::Update => Method::Update,
            JsonMethod::Delete => Method::Delete,
        }
    }
}
