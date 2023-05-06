use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum WorkspaceError {
    ProjectNotSelected,
    CollectionNotSelected,
    WorkspaceNotFound,
    ProjectsNotFound,
    ProjectNotFound,
}

impl Display for WorkspaceError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspaceError::ProjectNotSelected => {
                write!(f, "Project has to be selected to perform this action.")
            }
            WorkspaceError::CollectionNotSelected => {
                write!(f, "Collection has to be selected to perform this action.")
            }
            WorkspaceError::WorkspaceNotFound => write!(f, "Workspace not found."),
            WorkspaceError::ProjectsNotFound => write!(f, "Workspace has no projects."),
            WorkspaceError::ProjectNotFound => write!(f, "Workspace contains no such project."),
        }
    }
}

impl Error for WorkspaceError {}
