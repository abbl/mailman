use std::rc::Rc;

use crate::{models::workspace::Workspace, parsers::command::command_parser::CommandParser};

pub struct CliView {
    command_parser: Rc<CommandParser>,
}

impl CliView {
    pub fn new(command_parser: Rc<CommandParser>) -> Self {
        CliView { command_parser }
    }

    pub fn process_arguments(&self, args: Vec<String>) {
        match self.command_parser.parse(args) {
            Some((command_handler, arguments_map)) => {
                command_handler.handle_command(arguments_map, Workspace::new())
            }
            None => {
                eprintln!("Command not found");

                std::process::exit(1);
            }
        }
    }
}
