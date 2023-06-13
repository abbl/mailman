use std::{collections::HashMap, rc::Rc};

use crate::{
    models::workspace::Workspace,
    parsers::command::{
        command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
    },
    services::workspace_service::WorkspaceService,
    views::cli::cli_input::CliInput,
};

pub struct CreateWorkspaceCommand {
    workspace_service: Rc<WorkspaceService>,
}

impl CommandHandler for CreateWorkspaceCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>, workspace: Workspace) -> () {}

    fn processable_command(&self) -> ProcessableCommand {
        Command::new("projects")
            .add_subcommand(Command::new("list"))
            .to_processable_command()
    }
}

impl CreateWorkspaceCommand {
    pub fn new(
        cli_input: Rc<CliInput>,
        workspace_service: Rc<WorkspaceService>,
    ) -> CreateWorkspaceCommand {
        CreateWorkspaceCommand { workspace_service }
    }
}
