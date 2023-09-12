
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
