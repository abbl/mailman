use std::rc::Rc;

use crate::{
    errors::workspace_error::{LoadWorkspaceError, WorkspaceError},
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

    pub fn create_workspace(&self, project: Project) -> Workspace {
        let mut workspace = Workspace::new();
        let project_id = project.id().to_owned();

        workspace.add_project(project);
        workspace.select_project(&project_id).unwrap();

        workspace
    }

    pub fn load_workspace(&self) -> Result<Workspace, LoadWorkspaceError> {
        let projects = self.projects_storage.load_projects();
        let workspace = self.workspace_storage.load_workspace();

        match workspace {
            Some(mut workspace) => {
                match projects {
                    Some(projects) => {
                        let mut projects_iter = projects.into_iter();

                        while let Some(project) = projects_iter.next() {
                            workspace.add_project(project);
                        }
                    }
                    None => {}
                }

                return Ok(workspace);
            }
            None => Err(LoadWorkspaceError::WorkspaceNotFound),
        }
    }

    pub fn save_workspace(&self, workspace: Workspace) {
        self.projects_storage.save_projects(workspace.projects());
        self.workspace_storage.save_workspace(&workspace);
    }
}
