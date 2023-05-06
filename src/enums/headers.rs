use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Headers {
    ContentType,
}

#[derive(Serialize, Deserialize)]
pub enum ContentType {
    Text,
    Json,
    FormData,
}
