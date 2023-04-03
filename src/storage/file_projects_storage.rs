use std::{
    fs::{self, File},
    io::{Result as IoResult, Write},
};

use crate::models::project::Project;

use super::{json::json_project::JsonProject, projects_storage::ProjectsStorage};

pub struct FileProjectsStorage {}

const FILE_NAME: &str = "collections.json";

impl ProjectsStorage for FileProjectsStorage {
    fn load(&self) -> Vec<Project> {
        let json = self.load_json_from_file().unwrap();
        let projects = serde_json::from_str(&json);

        match projects {
            Ok(projects) => self.from_json(projects),
            Err(_) => panic!(),
        }
    }

    fn save(&self, project: Vec<Project>) -> () {
        let json = self.to_json(project);

        self.save_json_to_file(&json).ok();
    }
}

impl FileProjectsStorage {
    fn to_json(&self, projects: Vec<Project>) -> String {
        let json_projects: Vec<JsonProject> =
            projects.iter().map(|p| JsonProject::new(&p)).collect();

        match serde_json::to_string(&json_projects) {
            Ok(v) => v,
            Err(_err) => panic!(),
        }
    }

    fn from_json(&self, json_projects: Vec<JsonProject>) -> Vec<Project> {
        json_projects.iter().map(|p| p.to_project()).collect()
    }

    fn save_json_to_file(&self, json: &str) -> IoResult<()> {
        let mut file = File::create(FILE_NAME)?;

        file.write_all(&json.as_bytes())?;

        Ok(())
    }

    fn load_json_from_file(&self) -> IoResult<String> {
        let buffer = fs::read_to_string(FILE_NAME)?;

        Ok(buffer.to_string())
    }
}
