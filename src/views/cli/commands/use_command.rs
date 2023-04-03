use std::collections::HashMap;

use crate::parsers::command::{
    command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
};

pub struct UseCommand {
    processable_command: ProcessableCommand,
}

impl CommandHandler for UseCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>) -> () {
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
