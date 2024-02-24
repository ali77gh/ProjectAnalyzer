use std::fmt::Display;

//TODO use thiserror for this
pub enum AnalyzeErr {}

impl Display for AnalyzeErr {
    fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}
