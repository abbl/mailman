use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    enums::storage_type::StorageType,
    errors::project_error::ProjectError,
    utils::structures::tree::{Node, Tree},
};

use super::{
    collection::Collection,
    project_node::{IdentifiableProjectNode, ProjectNode},
};

#[derive(Deserialize, Serialize, Debug)]
pub struct Project {
    id: String,
    name: String,
    nodes: Tree<ProjectNode>,
    storage_type: StorageType,
}

impl Default for Project {
    fn default() -> Self {
        Project {
            id: Uuid::new_v4().to_string(),
            name: String::from(""),
            nodes: Tree::new(ProjectNode::Collection(Collection::new("root"))),
            storage_type: StorageType::Local,
        }
    }
}

impl Project {
    pub fn new(name: &str) -> Self {
        Project {
            name: name.to_string(),
            nodes: Tree::new(ProjectNode::Collection(Collection::new(name))),
            ..Project::default()
        }
    }

    pub fn new_with_id(id: &str, name: &str) -> Self {
        Project {
            id: id.to_owned(),
            name: name.to_owned(),
            ..Project::default()
        }
    }

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn root_collection(&self) -> &Collection {
        match self.nodes.root().data() {
            ProjectNode::Collection(collection) => return collection,
            ProjectNode::Request(_) => panic!(),
        }
    }

    pub fn add_to_collection(
        &mut self,
        collection: &Collection,
        node: ProjectNode,
    ) -> Result<(), ProjectError> {
        let collection_node = self.find_collection_node_mut(collection);

        match collection_node {
            Some(collection_node) => collection_node.add_child(node),
            None => return Err(ProjectError::FindCollectionError),
        };

        Ok(())
    }

    fn find_collection_node_mut(
        &mut self,
        collection: &Collection,
    ) -> Option<&mut Node<ProjectNode>> {
        self.nodes.find_mut(|n| n.id() == collection.id())
    }
}
