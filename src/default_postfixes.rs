use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use std::{borrow::Borrow, collections::HashSet};

#[derive(RustEmbed)]
#[folder = "./"]
#[include = "file_extensions.txt"]
struct Asset;
lazy_static! {
    pub static ref DEFAULT_POSTFIXES: HashSet<String> = {
        let content = Asset::get("file_extensions.txt").unwrap(); //read file at compile time
        let content = content.data;
        let content: &[u8] = content.borrow();
        let content = String::from_utf8(content.to_vec()).unwrap();

        let mut m = HashSet::new();
        for line in content.split('\n') {
            m.insert(line.to_string());
        }
        m
    };
}
