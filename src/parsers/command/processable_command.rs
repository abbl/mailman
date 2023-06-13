use std::collections::{HashMap, LinkedList};

use super::{command::Command, command_type::CommandType};

type CommandsArgumentsMap = HashMap<String, String>;

#[derive(Debug)]
pub struct ProcessableCommand {
    name: String,
    comment: String,
    command_type: CommandType,
    subcommands: Vec<ProcessableCommand>,
}

impl ProcessableCommand {
    pub fn new(
        name: String,
        comment: String,
        command_type: CommandType,
        mut subcommands: Vec<Command>,
    ) -> ProcessableCommand {
        ProcessableCommand {
            name,
            comment,
            command_type,
            subcommands: subcommands
                .drain(..)
                .map(|c| c.to_processable_command())
                .collect(),
        }
    }

    pub fn map_arguments_to_commands(
        &self,
        arguments: &Vec<String>,
    ) -> Result<CommandsArgumentsMap, String> {
        let mut commands = self.merge_commands();
        let mut commands_map: CommandsArgumentsMap = HashMap::new();

        for argument in arguments {
            match commands.pop_front() {
                Some(command) => {
                    if command.is_optional_type() {
                        continue;
                    }

                    if command.is_navigation_type() {
                        if command.matches_argument(&argument) {
                            continue;
                        }

                        return Err(String::from("No match found"));
                    }

                    if command.is_value_type() {
                        commands_map.insert(command.name.to_owned(), argument.clone());
                    }
                }
                None => {
                    return Err(String::from("No match found"));
                }
            }
        }

        Ok(commands_map)
    }

    pub fn command_type(&self) -> &CommandType {
        &self.command_type
    }

    fn matches_argument(&self, name: &str) -> bool {
        self.name == name
    }

    fn is_navigation_type(&self) -> bool {
        self.command_type == CommandType::Navigation
    }

    fn is_value_type(&self) -> bool {
        self.command_type == CommandType::Value
    }

    fn is_optional_type(&self) -> bool {
        self.command_type == CommandType::Optional
    }

    fn merge_commands(&self) -> LinkedList<&ProcessableCommand> {
        let mut commands: LinkedList<&ProcessableCommand> = LinkedList::new();

        commands.push_front(&self);
        self.subcommands.iter().for_each(|c| commands.push_back(c));

        commands
    }
}
