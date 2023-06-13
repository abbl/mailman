use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Header {
    pub is_enabled: bool,
    pub value: String,
}

impl Header {
    pub fn enable(&mut self) {
        self.is_enabled = true;
    }

    pub fn disable(&mut self) {
        self.is_enabled = false;
    }

    pub fn value(&self) -> &String {
        &self.value
    }

    pub fn new(value: &str) -> Header {
        Header {
            is_enabled: true,
            value: value.to_string(),
        }
    }
}
