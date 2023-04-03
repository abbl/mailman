use std::collections::HashMap;

use super::processable_command::ProcessableCommand;

pub type ArgumentsMap = HashMap<String, String>;

pub trait CommandHandler {
    fn handle_command(&self, arguments_map: HashMap<String, String>) -> ();
    fn processable_command(&self) -> &ProcessableCommand;
}
