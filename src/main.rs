use std::{
    path::Path,
    env
};

mod walker;
use walker::walk;

fn main() {
    
    match parse_input() {
        RequestType::SimplePostfix(_) => todo!(),
        RequestType::Invalid{msg} => todo!(),
        RequestType::Version => todo!(),
        RequestType::Update => todo!(), 
        RequestType::Help => {todo!()},
    };

    // if let Some(p) = prefix{
    //     let _ = walk(Path::new("./"), &|x|{
    //         if x.file_name().to_str().unwrap().ends_with(p){
    //             println!("{:?}", x.path());
    //         }
    //     });
    // }else{
    //     println!("TODO: show help")
    // }

}

fn parse_input() -> RequestType {
    let args: Vec<String> = env::args().collect();
    

    if let Some(arg1) = args.get(1){
        if arg1 == "--help"{
            return RequestType::Help;
        } else if arg1 == "--version"{
            return RequestType::Version;
        } else if arg1 == "update"{
            return RequestType::Update;
        } else if arg1.starts_with("--"){
            return RequestType::Invalid { msg: format!("unknown command --{}", arg1) };
        } else {
            return RequestType::SimplePostfix(arg1.clone());
        }
    }else{
        return RequestType::Help;
    }
}

enum RequestType{
    SimplePostfix(String),
    Invalid{msg: String},
    Help,
    Version,
    Update
}