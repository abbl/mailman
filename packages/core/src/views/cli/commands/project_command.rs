use std::{collections::HashMap, rc::Rc};

use crate::services::workspace_service::WorkspaceService;
use commands::{
    command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
};

pub struct ProjectCommand {
    workspace_service: Rc<WorkspaceService>,
}

impl CommandHandler for ProjectCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>) -> () {
        let workspace = self.workspace_service.load_workspace();

        match workspace.project() {
            Some(project) => println!("- {}", project.name()),
            None => println!("This workspace has no project selected."),
        }
    }

    fn processable_command(&self) -> ProcessableCommand {
        Command::new("project").to_processable_command()
    }
}

impl ProjectCommand {
    pub fn new(workspace_service: Rc<WorkspaceService>) -> ProjectCommand {
        ProjectCommand { workspace_service }
    }
}
