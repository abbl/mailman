use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum ProjectError {
    FindCollectionError,
}

impl Display for ProjectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ProjectError::FindCollectionError => {
                write!(f, "Collection could not be found within this project.")
            }
        }
    }
}

impl Error for ProjectError {}
