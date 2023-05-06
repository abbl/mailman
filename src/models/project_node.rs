use serde::{Deserialize, Serialize};

use super::{collection::Collection, request::Request};

#[derive(Serialize, Deserialize)]
pub enum ProjectNode {
    Request(Request),
    Collection(Collection),
}

pub trait IdentifiableProjectNode {
    fn name(&self) -> &str;
    fn id(&self) -> &str;
}

impl IdentifiableProjectNode for ProjectNode {
    fn name(&self) -> &str {
        match self {
            ProjectNode::Request(request) => request.name(),
            ProjectNode::Collection(collection) => collection.name(),
        }
    }

    fn id(&self) -> &str {
        match self {
            ProjectNode::Request(request) => request.id(),
            ProjectNode::Collection(collection) => collection.id(),
        }
    }
}
