pub mod enums;
pub mod http;
pub mod models;
pub mod parsers;
pub mod storage;
pub mod views;

use std::env;

use parsers::command::command_parser::CommandParser;
use views::cli::commands::use_command::UseCommand;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    let command_parser = CommandParser::new(vec![Box::new(UseCommand::new())]);

    if args.len() > 1 {
        args.remove(0);

        match command_parser.parse(args) {
            Some((command_handler, arguments_map)) => command_handler.handle_command(arguments_map),
            None => {
                eprintln!("Command not found");

                std::process::exit(1);
            }
        }
    } else {
        println!("<GUI View>");
    }
}
