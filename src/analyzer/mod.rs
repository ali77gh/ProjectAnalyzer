mod error;
pub(crate) mod result;
mod walker;

use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::PathBuf;
use tokio::sync::mpsc;
use walker::Walker;

use crate::arg_parser::MyArgs;

use self::error::AnalyzeErr;
use self::result::{AnalyzeResult, AnalyzeResultItem};

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

        let walker = Walker::new(
            self.args.root_dir().to_string(),
            self.args.ignore(),
            tx12,
            Some(post_set.clone()),
        );

        walker.start();

        let (tx23, mut rx23) = mpsc::channel::<FileWithPost>(100); // channel thread 2,3
                                                                   // thread2: file reader
        tokio::spawn(async move {
            while let Some(x) = rx12.recv().await {
                let postfix = x
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap_or("")
                    .split('.')
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

        let mut r = vec![];
        for postfix in post_set {
            r.push(AnalyzeResultItem::new(
                postfix.clone(),
                *file_counter.get(postfix.as_str()).unwrap(),
                *line_counter.get(postfix.as_str()).unwrap(),
            ));
        }
        Ok(AnalyzeResult::new(r))
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
