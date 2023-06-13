use serde::{Deserialize, Serialize};

use crate::errors::workspace_error::WorkspaceError;

use super::{collection::Collection, project::Project};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    project_id: Option<String>,
    collection_id: Option<String>,

    #[serde(skip_serializing, skip_deserializing)]
    projects: Vec<Project>,
}

impl Default for Workspace {
    fn default() -> Self {
        Workspace {
            projects: vec![],
            collection_id: None,
            project_id: None,
        }
    }
}

impl Workspace {
    pub fn new() -> Self {
        Workspace::default()
    }

    pub fn add_project(mut self, project: Project) -> Workspace {
        self.projects.push(project);

        self
    }

    pub fn add_default_project(mut self, project: Project) -> Workspace {
        let project_id = &project.id().to_owned();

        self.projects.push(project);
        self.select_project(project_id).unwrap();

        self
    }

    pub fn select_project(&mut self, project_id: &str) -> Result<(), WorkspaceError> {
        let project = self.find_project(project_id);

        match project {
            Some(project) => {
                let project_id = project.id().to_owned();
                let collection_id = project.root_collection().id().to_owned();

                self.project_id = Some(project_id);
                self.collection_id = Some(collection_id);

                return Ok(());
            }
            None => {}
        }

        Err(WorkspaceError::ProjectNotFound)
    }

    pub fn project_and_collection_ids(&self) -> (Option<&str>, Option<&str>) {
        (self.project_id.as_deref(), self.collection_id.as_deref())
    }

    pub fn project(&self) -> Option<&Project> {
        self.projects
            .iter()
            .find(|p| Some(p.id()) == self.project_id.as_deref())
    }

    pub fn projects(&self) -> &Vec<Project> {
        &self.projects
    }

    pub fn collection(&self) -> Option<&Collection> {
        match self.project() {
            Some(project) => Some(project.root_collection()),
            None => None,
        }
    }

    fn is_project_in_workspace(&self, project: &Project) -> bool {
        self.projects.iter().any(|p| p.id() == project.id())
    }

    fn find_project(&self, project_id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id() == project_id)
    }
}
