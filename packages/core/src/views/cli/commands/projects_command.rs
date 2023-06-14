use std::{collections::HashMap, rc::Rc};

use crate::services::workspace_service::WorkspaceService;
use commands::{
    command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
};

pub struct ProjectsCommand {
    workspace_service: Rc<WorkspaceService>,
}

impl CommandHandler for ProjectsCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>) -> () {
        let workspace = self.workspace_service.load_workspace();

        for project in workspace.projects() {
            println!("- {}", project.name());
        }
    }

    fn processable_command(&self) -> ProcessableCommand {
        Command::new("projects").to_processable_command()
    }
}

impl ProjectsCommand {
    pub fn new(workspace_service: Rc<WorkspaceService>) -> ProjectsCommand {
        ProjectsCommand { workspace_service }
    }
}
