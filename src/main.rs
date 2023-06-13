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

use views::cli::{cli_view::CliView, commands::create_workspace_command::CreateWorkspaceCommand};

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

    if args.len() > 1 {
        let use_command = UseCommand::new();
        let cli_input = Rc::new(CliInput {});
        let create_workspace_command =
            CreateWorkspaceCommand::new(cli_input, workspace_service.clone());
        let command_parser = CommandParser::new(vec![
            Box::new(use_command),
            Box::new(create_workspace_command),
        ]);

        CliView::new(Rc::new(command_parser)).process_arguments(args);
    }
}
