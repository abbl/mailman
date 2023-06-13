use std::{collections::HashMap, rc::Rc};

use crate::{
    models::{project::Project, workspace::Workspace},
    parsers::command::{
        command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
    },
    services::workspace_service::WorkspaceService,
    views::cli::cli_input::{CliInput, CliInputSchema},
};

pub struct CreateWorkspaceCommand {
    workspace_service: Rc<WorkspaceService>,
    cli_input: Rc<CliInput>,
}

impl CommandHandler for CreateWorkspaceCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>, workspace: Workspace) -> () {
        let project_name =
            CliInput::read_string(CliInputSchema::new().add_description("Insert project name:"));

        self.workspace_service.save_workspace(
            self.workspace_service
                .load_workspace()
                .add_default_project(Project::new(&project_name)),
        );
    }

    fn processable_command(&self) -> ProcessableCommand {
        Command::new("create")
            .add_subcommand(Command::new("workspace"))
            .to_processable_command()
    }
}

impl CreateWorkspaceCommand {
    pub fn new(
        cli_input: Rc<CliInput>,
        workspace_service: Rc<WorkspaceService>,
    ) -> CreateWorkspaceCommand {
        CreateWorkspaceCommand {
            workspace_service,
            cli_input,
        }
    }
}
