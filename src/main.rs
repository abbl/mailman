#![allow(dead_code, unused_variables)]

mod enums;
mod errors;
mod http;
mod models;
mod parsers;
mod services;
mod storage;
mod utils;
mod views;

use std::{env, rc::Rc};

use views::cli::{
    cli_view::CliView,
    commands::{
        create_project_command::CreateProjectCommand, project_command::ProjectCommand,
        projects_command::ProjectsCommand,
    },
};

use crate::{
    parsers::command::command_parser::CommandParser,
    services::workspace_service::WorkspaceService,
    storage::{
        file::file_storage::FileStorage, storable_projects::StorableProjects,
        storable_workspace::StorableWorkspace,
    },
    views::cli::{cli_input::CliInput, commands::use_command::UseCommand},
};

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);

    let storable_projects: Rc<dyn StorableProjects> = Rc::new(FileStorage {});
    let storable_workspace: Rc<dyn StorableWorkspace> = Rc::new(FileStorage {});

    let workspace_service = Rc::new(WorkspaceService::new(
        storable_projects.clone(),
        storable_workspace.clone(),
    ));

    if args.len() >= 1 {
        let cli_input = Rc::new(CliInput {});

        let command_parser = CommandParser::new(vec![
            Box::new(UseCommand::new()),
            Box::new(CreateProjectCommand::new(
                cli_input,
                workspace_service.clone(),
            )),
            Box::new(ProjectsCommand::new(workspace_service.clone())),
            Box::new(ProjectCommand::new(workspace_service.clone())),
        ]);

        CliView::new(Rc::new(command_parser)).process_arguments(args);
    }
}
