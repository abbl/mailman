use std::{collections::HashMap, io};

use crate::{
    models::workspace::Workspace,
    parsers::command::{
        command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
    },
};

pub struct UseCommand {
    processable_command: ProcessableCommand,
}

impl CommandHandler for UseCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>, workspace: Workspace) -> () {
        println!("Select the project id:");

        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string).unwrap();

        println!("{}", input_string);

        todo!()
    }

    fn processable_command(&self) -> &ProcessableCommand {
        &self.processable_command
    }
}

impl UseCommand {
    pub fn new() -> UseCommand {
        let command =
            Command::new("use").add_subcommand(Command::new("collection_name").expect_value());

        UseCommand {
            processable_command: command.to_processable_command(),
        }
    }
}
