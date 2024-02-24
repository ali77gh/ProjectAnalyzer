mod error;
pub(crate) mod result;
mod walker;

use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::fs;
use std::os::unix::thread;
use std::path::PathBuf;
use std::sync::mpsc::{Receiver, Sender};
use std::time::Duration;
use std::{fs::DirEntry, path::Path};
use tokio::sync::mpsc;
use walker::walk;

use crate::arg_parser::MyArgs;

use self::error::AnalyzeErr;
use self::result::AnalyzeResult;

pub struct Analyzer<'a> {
    args: &'a MyArgs,
}

impl<'a> Analyzer<'a> {
    pub fn new(args: &'a MyArgs) -> Self {
        Self { args }
    }

    pub async fn analyze(&self) -> Result<AnalyzeResult, AnalyzeErr> {
        let mut file_counter: HashMap<String, u64> = HashMap::new();
        let mut line_counter: HashMap<String, u64> = HashMap::new();
        let mut post_set: HashSet<String> = HashSet::new();

        let postfixes = self.args.postfixes().unwrap(); // TODO auto detect if None passed

        for p in postfixes {
            file_counter.insert(p.clone(), 0);
            line_counter.insert(p.clone(), 0);
            post_set.insert(p.clone());
        }

        let (tx12, mut rx12) = mpsc::channel::<PathBuf>(1000); // channel thread 1,2
                                                               // thread1: walk and filter by postfix
        let post_set_t1 = post_set.clone();
        tokio::spawn(async move {
            let (s, r) = std::sync::mpsc::channel();
            let mut cb = move |x: &DirEntry| {
                let binding = x.file_name();
                let postfix = binding
                    .to_str()
                    .unwrap_or("")
                    .split(".")
                    .last()
                    .unwrap_or("nothing_file");
                if post_set_t1.contains(postfix) {
                    s.send(x.path()).unwrap();
                }
            };

            std::thread::spawn(move || {
                let _ = walk(Path::new("./"), &mut cb);
            });

            while let Ok(path) = r.recv() {
                tx12.send(path).await.unwrap();
            }
        });

        let (tx23, mut rx23) = mpsc::channel::<FileWithPost>(100); // channel thread 2,3
                                                                   // thread2: file reader
        tokio::spawn(async move {
            while let Some(x) = rx12.recv().await {
                let postfix = x
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap_or("")
                    .split(".")
                    .last()
                    .unwrap_or("nothing_file");

                let content = fs::read(x.clone()).unwrap_or_else(|_| {
                    println!("can't read file:{}", x.to_str().unwrap());
                    vec![]
                });
                tx23.send(FileWithPost::new(postfix.to_string(), content))
                    .await
                    .unwrap();
            }
        });

        // main thread: counting
        // let nl = b'\n';
        // let nl = &&nl; // pre process dereference
        while let Some(x) = rx23.recv().await {
            let fc = file_counter.get_mut(x.postfix.as_str()).unwrap();
            let lc = line_counter.get_mut(x.postfix.as_str()).unwrap();
            *fc += 1;
            *lc += x.bytes.iter().filter(|byte| **byte == b'\n').count() as u64;
        }

        Ok(AnalyzeResult::new(file_counter, line_counter, post_set))
    }
}

struct FileWithPost {
    postfix: String,
    bytes: Vec<u8>,
}

impl FileWithPost {
    fn new(postfix: String, bytes: Vec<u8>) -> Self {
        FileWithPost { postfix, bytes }
    }
}
