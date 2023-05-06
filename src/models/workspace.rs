use serde::{Deserialize, Serialize};

use crate::errors::workspace_error::WorkspaceError;

use super::{collection::Collection, project::Project};

#[derive(Serialize, Deserialize)]
pub struct Workspace {
    project_id: Option<String>,
    collection_id: Option<String>,
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

    pub fn add_project(&mut self, project: Project) {
        if !self.is_project_in_workspace(&project) {
            self.projects.push(project);
        }
    }

    pub fn select_project(&mut self, project: &Project) -> Result<(), WorkspaceError> {
        if self.is_project_in_workspace(project) {
            self.project_id = Some(project.id().to_owned());
            self.collection_id = Some(project.root_collection().id().to_owned());

            return Ok(());
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

    pub fn collection(&self) -> Option<&Collection> {
        match self.project() {
            Some(project) => Some(project.root_collection()),
            None => None,
        }
    }

    fn is_project_in_workspace(&self, project: &Project) -> bool {
        self.projects.iter().any(|p| p.id() == project.id())
    }
}
