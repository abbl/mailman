use std::{
    error::Error,
    fs::{self, File},
    io::{Result as IoResult, Write},
};

use crate::{
    models::{project::Project, workspace::Workspace},
    storage::{storable_projects::StorableProjects, storable_workspace::StorableWorkspace},
};

pub struct FileStorage {}

const PROJECTS_FILE_NAME: &str = "collections.json";
const WORKSPACE_FILE_NAME: &str = "workspace.json";

impl StorableProjects for FileStorage {
    fn load_projects(&self) -> Option<Vec<Project>> {
        match self.load_json_from_file(PROJECTS_FILE_NAME) {
            Ok(json) => serde_json::from_str(&json).unwrap(),
            Err(_) => return None,
        }
    }

    fn save_projects(&self, project: Vec<Project>) -> () {
        let json = self.to_json_projects(project);

        self.save_json_to_file(PROJECTS_FILE_NAME, &json).ok();
    }
}

impl StorableWorkspace for FileStorage {
    fn load_workspace(&self) -> Option<Workspace> {
        match self.load_json_from_file(WORKSPACE_FILE_NAME) {
            Ok(json) => serde_json::from_str(&json).unwrap(),
            Err(_) => return None,
        }
    }

    fn save_workspace(&self, workspace: Workspace) -> () {
        let json = self.to_json_workspace(workspace);

        self.save_json_to_file(&WORKSPACE_FILE_NAME, &json).ok();
    }
}

impl FileStorage {
    fn to_json_projects(&self, projects: Vec<Project>) -> String {
        match serde_json::to_string(&projects) {
            Ok(v) => v,
            Err(_err) => panic!(),
        }
    }

    fn to_json_workspace(&self, workspace: Workspace) -> String {
        match serde_json::to_string(&workspace) {
            Ok(v) => v,
            Err(_err) => panic!(),
        }
    }

    fn save_json_to_file(&self, path: &str, json: &str) -> IoResult<()> {
        let mut file = File::create(path)?;

        file.write_all(&json.as_bytes())?;

        Ok(())
    }

    fn load_json_from_file(&self, path: &str) -> IoResult<String> {
        let buffer = fs::read_to_string(path)?;

        Ok(buffer.to_string())
    }
}
