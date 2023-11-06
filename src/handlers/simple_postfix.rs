use std::{path::Path, fs::DirEntry, sync::mpsc, thread};
use crate::utils::walker::walk;
use std::fs;
use std::path::PathBuf;

pub fn simple_postfix_handler(postfix: &str){
    let mut file_counter: usize = 0;
    let mut line_counter: usize = 0;

    let thread_postfix = postfix.to_owned();
    let banner_postfix = postfix.to_owned();

    let (tx12, rx12) = mpsc::channel::<Option<PathBuf>>(); // channel thread 1,2
    thread::spawn(move ||{
        let filter_by = format!(".{}", thread_postfix);
        let mut cb = |x:&DirEntry|{
            if x.file_name().to_str().unwrap().ends_with(&filter_by){
                tx12.send(Some(x.path())).unwrap();
            }
        };
        let _ = walk(Path::new("./"), &mut cb);
        tx12.send(None).unwrap();
    });

    let (tx23, rx23) = mpsc::channel::<Option<Vec<u8>>>();  // channel thread 2,3
    // file provider thread
    thread::spawn(move || {
        while let Some(x) = rx12.recv().unwrap(){
            let content = fs::read(x.clone())
                .unwrap_or_else(|_|{ 
                    println!("can't read file:{}", x.to_str().unwrap()); vec![]
            });
            tx23.send(Some(content)).unwrap();
        }
        tx23.send(None).unwrap();
    });

    // counter thread
    let nl = b'\n';
    let nl = &&nl; // pre process dereference
    while let Some(x) = rx23.recv().unwrap(){
        file_counter += 1;
        line_counter += x.iter().filter(|byte|byte==nl).count();
    }

    let division = line_counter/file_counter;

    println!("┌───────────────────────────────────────────────┐");
    println!("│                ProjectAnalyzer                │");
    println!("│                                               │");
    println!("│ https://github.com/ali77gh/ProjectAnalyzer    │");
    println!("│                                               │");
    println!("│ searching...                                  │");
    println!("│ you have {} {} files           \t\t│", file_counter,banner_postfix);
    println!("│ you have {} lines of {}      \t\t│", line_counter,banner_postfix);
    println!("│ average lines per file: {}       \t\t│", division);
    println!("│                                               │");
    println!("└───────────────────────────────────────────────┘");

}