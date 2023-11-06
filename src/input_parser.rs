
use std::env;

pub enum RequestType{
    SimplePostfix(String),
    Invalid{msg: String},
    Help,
    Version,
    Update
}

pub fn parse_input() -> RequestType {
    let args: Vec<String> = env::args().collect();
    
    if let Some(arg1) = args.get(1){
        if arg1 == "--help" || arg1=="-h"{
            RequestType::Help
        } else if arg1 == "--version" || arg1=="-v" {
            return RequestType::Version;
        } else if arg1 == "--update" || arg1 == "-u"{
            return RequestType::Update;
        } else if arg1.starts_with("--"){
            return RequestType::Invalid { msg: format!("  * Error: unknown command {} *  ", arg1) };
        } else {
            return RequestType::SimplePostfix(arg1.clone());
        }
    }else{
        RequestType::Help
    }
}
