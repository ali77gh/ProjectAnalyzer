use std::path::Path;
use crate::utils::walker::walk;

pub fn simple_postfix_handler(postfix: &String){
    let _ = walk(Path::new("./"), &|x|{
        if x.file_name().to_str().unwrap().ends_with(postfix){
            println!("{:?}", x.path());
        }
    });
}