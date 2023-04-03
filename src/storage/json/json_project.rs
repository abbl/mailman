use serde::{Deserialize, Serialize};

use crate::models::project::Project;

use super::json_collection::JsonCollection;

#[derive(Serialize, Deserialize)]
pub struct JsonProject {
    name: String,
    collections: Vec<JsonCollection>,
}

impl JsonProject {
    pub fn new(project: &Project) -> JsonProject {
        JsonProject {
            name: project.name().to_string(),
            collections: project
                .collections()
                .iter()
                .map(|c| JsonCollection::new(c))
                .collect(),
        }
    }

    pub fn to_project(&self) -> Project {
        let mut project = Project::new(&self.name);

        for json_collection in &self.collections {
            let mut collection = json_collection.to_collection();

            loop {
                match project.add_collection(&collection) {
                    Ok(_) => break,
                    Err(_) => collection.deduplicate_name(),
                };
            }
        }

        project
    }
}
