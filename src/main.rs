
mod input_parser;
mod handlers;
mod utils;

use input_parser::{ RequestType, parse_input };
use utils::show_help::show_help;
use handlers::{
    simple_postfix::simple_postfix_handler,
    update::update_handler,
    version::version_handler
};

fn main() {

    match parse_input() {
        RequestType::SimplePostfix(postfixes) => simple_postfix_handler(postfixes),
        RequestType::Invalid{ msg } => {
            println!("{}", msg);
            show_help();
        },
        RequestType::Version => version_handler(),
        RequestType::Update => update_handler(), 
        RequestType::Help => show_help(),
    };
}
