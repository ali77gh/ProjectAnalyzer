use std::{path::Path, fs::DirEntry};
use crate::utils::walker::walk;
use std::fs;

pub fn simple_postfix_handler(postfix: &String){
    let mut file_counter: usize = 0;
    let mut line_counter: usize = 0;

    let filter_by = format!(".{}",postfix);

    let mut cb = |x:&DirEntry|{
        if x.file_name().to_str().unwrap().ends_with(&filter_by){
            file_counter+=1;
            let content = fs::read_to_string(x.path())
                .unwrap_or_else(|_|{println!("can't read file:{}",x.path().to_str().unwrap()); "".to_string()});

            line_counter += content.split("\n").count()-1;
        }
    };

    let _ = walk(Path::new("./"), &mut cb);

    println!("┌───────────────────────────────────────────────┐");
    println!("│                ProjectAnalyzer                │");
    println!("│                                               │");
    println!("│ https://github.com/ali77gh/ProjectAnalyzer    │");
    println!("│                                               │");
    println!("│ searching...                                  │");
    println!("│ you have {} {} files           \t\t│",file_counter,postfix);
    println!("│ you have {} lines of {}      \t\t│",line_counter,postfix);
    println!("│ lines per file average: 104.0                 │");
    println!("│                                               │");
    println!("└───────────────────────────────────────────────┘");

}