use crate::models::project::Project;

pub trait StorableProjects {
    fn load_projects(&self) -> Option<Vec<Project>>;
    fn save_projects(&self, projects: Vec<Project>) -> ();
}
