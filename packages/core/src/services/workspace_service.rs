use std::rc::Rc;

use crate::{
    models::workspace::Workspace,
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

    pub fn load_workspace(&self) -> Workspace {
        let projects = self.projects_storage.load_projects().unwrap_or(Vec::new());
        let mut workspace = self
            .workspace_storage
            .load_workspace()
            .unwrap_or(Workspace::new());

        let mut projects_iter = projects.into_iter();

        while let Some(project) = projects_iter.next() {
            workspace = workspace.add_project(project);
        }

        workspace
    }

    pub fn save_workspace(&self, workspace: Workspace) {
        self.projects_storage.save_projects(workspace.projects());
        self.workspace_storage.save_workspace(&workspace);
    }
}
