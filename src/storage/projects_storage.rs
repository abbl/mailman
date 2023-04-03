use crate::models::project::Project;

pub trait ProjectsStorage {
    fn load(&self) -> Vec<Project>;
    fn save(&self, projects: Vec<Project>) -> ();
}
