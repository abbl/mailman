use super::{
    command_handler::{ArgumentsMap, CommandHandler},
    command_type::CommandType,
};

pub struct CommandParser {
    commands_handlers: Vec<Box<dyn CommandHandler>>,
}

impl CommandParser {
    pub fn new(commands_handlers: Vec<Box<dyn CommandHandler>>) -> CommandParser {
        let mut parser = CommandParser { commands_handlers };

        parser.commands_handlers = CommandParser::sort_commands(&mut parser.commands_handlers);

        parser
    }

    pub fn parse(
        &self,
        arguments: Vec<String>,
    ) -> Option<(&Box<dyn CommandHandler>, ArgumentsMap)> {
        for command_handler in &self.commands_handlers {
            let processable_command = command_handler.processable_command();

            match processable_command.map_arguments_to_commands(&arguments) {
                Ok(arguments_map) => {
                    return Some((command_handler, arguments_map));
                }
                Err(_) => continue,
            }
        }

        None
    }

    fn sort_commands(commands: &mut Vec<Box<dyn CommandHandler>>) -> Vec<Box<dyn CommandHandler>> {
        let mut sorted_commands: Vec<Box<dyn CommandHandler>> = Vec::new();
        let sorting_order = [
            CommandType::Navigation,
            CommandType::Optional,
            CommandType::Value,
        ];

        for command_type in sorting_order {
            sorted_commands
                .append(CommandParser::extract_commands_by_type(commands, command_type).as_mut());
        }

        sorted_commands
    }

    fn extract_commands_by_type(
        source: &mut Vec<Box<dyn CommandHandler>>,
        command_type: CommandType,
    ) -> Vec<Box<dyn CommandHandler>> {
        let mut extracted_commands: Vec<Box<dyn CommandHandler>> = Vec::new();
        let mut indexes: Vec<usize> = Vec::new();

        for (index, command_handler) in source.iter().enumerate() {
            if command_handler.processable_command().command_type() == &command_type {
                indexes.push(index);
            }
        }

        for index in indexes {
            let command = source.remove(index);

            extracted_commands.push(command);
        }

        extracted_commands
    }
}
