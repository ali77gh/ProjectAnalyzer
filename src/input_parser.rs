
use std::env;

pub enum RequestType{
    SimplePostfix(Vec<String>),
    Invalid{msg: String},
    Help,
    Version,
    Update
}

pub fn parse_input() -> RequestType{
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
            let tmp = arg1.clone();
            let tmp = tmp.split(",");
            let tmp:Vec<String> = tmp.map(|s|s.to_string()).collect();
            return RequestType::SimplePostfix(tmp);
        }
    }else{
        RequestType::Help
    }
}
