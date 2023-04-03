pub struct CliState {
    pub project_name: String,
    pub collection_name: String,
}

impl CliState {
    pub fn new(project_name: &str, collection_name: &str) -> CliState {
        CliState {
            project_name: project_name.to_string(),
            collection_name: collection_name.to_string(),
        }
    }
}
