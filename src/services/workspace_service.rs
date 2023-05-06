use std::rc::Rc;

use crate::{
    errors::workspace_error::WorkspaceError,
    models::{project::Project, workspace::Workspace},
    storage::{storable_projects::StorableProjects, storable_workspace::StorableWorkspace},
};

pub struct WorkspaceService {
    projects_storage: Rc<dyn StorableProjects>,
    workspace_storage: Rc<dyn StorableWorkspace>,
}

impl WorkspaceService {
    pub fn new(
        projects_storage: Rc<dyn StorableProjects>,
        workspace_storage: Rc<dyn StorableWorkspace>,
    ) -> WorkspaceService {
        WorkspaceService {
            projects_storage,
            workspace_storage,
        }
    }

    pub fn create_workspace(project: Project) -> Workspace {
        let mut workspace = Workspace::new();

        panic!()
    }

    pub fn load_workspace(&self) -> Result<Workspace, WorkspaceError> {
        let workspace = self.workspace_storage.load_workspace();

        match workspace {
            Some(mut workspace) => return Ok(workspace),
            None => Err(WorkspaceError::WorkspaceNotFound),
        }
    }

    pub fn save_workspace(&self) {}
}
