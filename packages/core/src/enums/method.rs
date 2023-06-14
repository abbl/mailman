use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub enum Method {
    Get,
    Post,
    Patch,
    Put,
    Update,
    Delete,
}
