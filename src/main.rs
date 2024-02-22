mod arg_parser;

use clap::Parser;

use arg_parser::MyArgs;

mod handlers;
mod utils;

use handlers::{simple_postfix::simple_postfix_handler, update::update_handler};

fn main() {
    let args = MyArgs::parse();

    if let Some(command) = args.command() {
        match command {
            arg_parser::MyCommands::Update => update_handler(),
        }
    } else {
        if let Some(postfixes) = args.postfixes() {
            simple_postfix_handler(postfixes);
        } else {
            println!("automatic source file detection comming soon");
        }
    }
}
