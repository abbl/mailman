use serde::{Deserialize, Serialize};

use crate::models::header::Header;

#[derive(Serialize, Deserialize)]
pub struct JsonHeader {
    pub is_enabled: bool,
    pub value: String,
}

impl JsonHeader {
    pub fn new(header: &Header) -> JsonHeader {
        JsonHeader {
            is_enabled: header.is_enabled.clone(),
            value: header.value.clone(),
        }
    }
}
