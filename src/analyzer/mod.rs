mod counter;
mod error;
pub(crate) mod result;
mod walker;

use std::path::PathBuf;
use tokio::sync::mpsc;
use walker::Walker;

use crate::arg_parser::MyArgs;

use self::counter::Counter;
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
        let (tx12, rx12) = mpsc::channel::<PathBuf>(1000); // channel thread 1,2
        let (tx23, mut rx23) = mpsc::channel::<AnalyzeResult>(1); // channel thread 2,3

        Walker::new(self.args, tx12).start();
        Counter::new(rx12, tx23).start();

        Ok(rx23.recv().await.unwrap())
    }
}
