use std::{collections::HashMap, io};

use crate::parsers::command::{
    command::Command, command_handler::CommandHandler, processable_command::ProcessableCommand,
};

pub struct UseCommand {}

impl CommandHandler for UseCommand {
    fn handle_command(&self, arguments_map: HashMap<String, String>) -> () {
        println!("Select the project id:");

        let mut input_string = String::new();

        io::stdin().read_line(&mut input_string).unwrap();

        println!("{}", input_string);

        todo!()
    }

    fn processable_command(&self) -> ProcessableCommand {
        Command::new("use")
            .add_subcommand(Command::new("collection_name").expect_value())
            .to_processable_command()
    }
}

impl UseCommand {
    pub fn new() -> UseCommand {
        UseCommand {}
    }
}
