mod analyzer;
mod arg_parser;
mod default_ignore;
mod ui;
mod update;

use std::{io, path::Path, sync::mpsc};

use analyzer::Analyzer;
use arg_parser::MyArgs;
use clap::Parser;

use crate::ui::chart::DrawableChart;

use update::update_handler;

use notify::{RecursiveMode, Watcher};

#[tokio::main]
async fn main() {
    let args = MyArgs::parse();

    if let Some(command) = args.command() {
        match command {
            arg_parser::MyCommands::Update => update_handler(),
            arg_parser::MyCommands::Watch => watch(&args).await,
        }
    } else {
        analyze_and_show(&args).await;
    }
}

async fn analyze_and_show(args: &MyArgs) {
    let analyzer = Analyzer::new(args);
    match analyzer.analyze().await {
        Ok(result) => {
            if args.json() {
                println!("{}", serde_json::to_string(&result).unwrap());
            } else {
                println!("{}", result.to_string());
                if result.iter().len() > 1 {
                    result.draw();
                }
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    };
}

async fn watch(args: &MyArgs) {
    clearscreen::clear().unwrap();
    analyze_and_show(args).await;

    let (tx, rx) = mpsc::channel::<()>(); // channel thread 1,2

    let mut watcher = notify::recommended_watcher(move |res| match res {
        Ok(_) => {
            clearscreen::clear().unwrap();
            tx.send(()).unwrap();
        }
        Err(e) => println!("watch error: {:?}", e),
    })
    .unwrap();
    let ar = args.clone();
    tokio::spawn(async move {
        while rx.recv().is_ok() {
            analyze_and_show(&ar).await;
        }
    });
    watcher
        .watch(Path::new(args.root_dir()), RecursiveMode::Recursive)
        .unwrap();

    //prevent exist
    let mut buffer = String::new();
    io::stdin().read_line(&mut buffer).unwrap();
}
