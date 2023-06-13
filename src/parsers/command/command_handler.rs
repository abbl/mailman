use std::collections::HashMap;

use crate::models::workspace::Workspace;

use super::processable_command::ProcessableCommand;

pub type ArgumentsMap = HashMap<String, String>;

pub trait CommandHandler {
    fn handle_command(&self, arguments_map: HashMap<String, String>, workspace: Workspace);
    fn processable_command(&self) -> ProcessableCommand;
}
