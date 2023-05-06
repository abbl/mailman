use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::request::Request;

#[derive(Clone, Serialize, Deserialize)]
pub struct Collection {
    id: String,
    name: String,
    requests: Vec<Request>,
}

impl Default for Collection {
    fn default() -> Self {
        Collection {
            id: Uuid::new_v4().to_string(),
            name: String::from(""),
            requests: Vec::new(),
        }
    }
}

impl Collection {
    pub fn new(name: &str) -> Self {
        Collection {
            name: name.to_owned(),
            ..Collection::default()
        }
    }

    pub fn new_with_id(id: &str, name: &str) -> Self {
        Collection {
            id: id.to_owned(),
            name: name.to_owned(),
            ..Collection::default()
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn requests(&self) -> &Vec<Request> {
        &self.requests
    }

    pub fn add_request(&mut self, request: Request) {
        self.requests.push(request);
    }

    pub fn deduplicate_name(&mut self) -> () {
        self.name = self.name.to_string() + "_copy";
    }
}
