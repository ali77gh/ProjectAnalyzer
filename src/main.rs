mod analyzer;
mod arg_parser;
mod ui;
mod update;

use analyzer::Analyzer;
use arg_parser::MyArgs;
use clap::Parser;

use crate::ui::chart::DrawableChart;

use update::update_handler;

fn main() {
    let args = MyArgs::parse();

    if let Some(command) = args.command() {
        match command {
            arg_parser::MyCommands::Update => update_handler(),
        }
    } else {
        let analyzer = Analyzer::new(&args);
        match analyzer.analyze() {
            Ok(result) => {
                println!("{}", result.to_string());
                if result.file_counter().len() > 1 {
                    result.draw();
                }
            }
            Err(err) => {
                println!("{}", err);
            }
        };
    }
}
