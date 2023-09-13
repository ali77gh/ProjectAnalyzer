
use webbrowser;

const GITHUB_ADDR: &str = "https://github.com/ali77gh/ProjectAnalyzer/"; 


pub fn update_handler(){


            if webbrowser::open(GITHUB_ADDR).is_ok() {
                println!("checkout: {}",GITHUB_ADDR)
            }else{

            }
}