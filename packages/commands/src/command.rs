use super::{command_type::CommandType, processable_command::ProcessableCommand};

pub struct Command {
    name: String,
    comment: String,
    command_type: CommandType,
    subcommands: Vec<Command>,
}

impl Command {
    pub fn new(name: &str) -> Command {
        Command {
            name: name.to_owned(),
            comment: String::from(""),
            command_type: CommandType::Navigation,
            subcommands: Vec::new(),
        }
    }

    pub fn expect_value(mut self) -> Command {
        self.command_type = CommandType::Value;

        self
    }

    pub fn comment(mut self, comment: &str) -> Command {
        self.comment = comment.to_owned();

        self
    }

    pub fn add_subcommand(mut self, subcommand: Command) -> Command {
        self.subcommands.push(subcommand);

        self
    }

    pub fn to_processable_command(self) -> ProcessableCommand {
        ProcessableCommand::new(self.name, self.comment, self.command_type, self.subcommands)
    }
}
