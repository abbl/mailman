use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Update,
    Delete,
}
