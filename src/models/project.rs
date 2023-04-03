use super::collection::Collection;

pub struct Project {
    name: String,
    collections: Vec<Collection>,
}

impl Project {
    pub fn new(name: &str) -> Project {
        Project {
            name: name.to_string(),
            collections: Vec::new(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn collections(&self) -> &Vec<Collection> {
        &self.collections
    }

    pub fn add_collection(&mut self, collection: &Collection) -> Result<(), String> {
        if self.is_collection_name_free(&collection.name) {
            self.collections.push(collection.clone());

            Ok(())
        } else {
            Err("Collection with name ".to_string()
                + &collection.name
                + " already exists in this project")
        }
    }

    pub fn remove_collection(&mut self, collection_name: &str) -> () {
        if let Some(index) = self
            .collections
            .iter()
            .position(|c| c.name == collection_name)
        {
            self.collections.remove(index);
        }
    }

    fn is_collection_name_free(&self, name: &str) -> bool {
        for collection in &self.collections {
            if collection.name == name {
                return false;
            }
        }

        true
    }
}
