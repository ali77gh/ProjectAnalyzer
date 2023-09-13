
mod input_parser;
mod handlers;
mod utils;

use input_parser::{ RequestType, parse_input };
use handlers::{
    simple_postfix::simple_postfix_handler,
    update::update_handler
};
use utils::show_help::show_help;

fn main() {

    match parse_input() {
        RequestType::SimplePostfix(postfix) => simple_postfix_handler(&postfix),
        RequestType::Invalid{ msg } => {
            println!("{}",msg);
            show_help();
        },
        RequestType::Version => todo!(),
        RequestType::Update => update_handler(), 
        RequestType::Help => show_help(),
    };
}
