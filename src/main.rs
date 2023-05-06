#![allow(dead_code, unused_variables)]

mod enums;
mod errors;
mod http;
mod models;
mod parsers;
mod services;
mod storage;
mod utils;
mod views;

use std::{env, rc::Rc};

use views::cli::cli_view::CliView;

use crate::{
    parsers::command::command_parser::CommandParser, views::cli::commands::use_command::UseCommand,
};

fn main() {
    let mut args: Vec<String> = env::args().collect();

    args.remove(0);

    if args.len() > 1 {
        let use_command = UseCommand::new();
        let command_parser = CommandParser::new(vec![Box::new(use_command)]);

        CliView::new(Rc::new(command_parser)).process_arguments(args);
    } else {
        println!("<GUI View>");
    }
}
