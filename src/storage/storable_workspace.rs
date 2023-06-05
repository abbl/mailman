use crate::models::workspace::Workspace;

pub trait StorableWorkspace {
    fn load_workspace(&self) -> Option<Workspace>;
    fn save_workspace(&self, workspace: &Workspace) -> ();
}
