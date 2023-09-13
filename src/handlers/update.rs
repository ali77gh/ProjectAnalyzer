
use webbrowser;

const GITHUB_ADDR: &str = "https://github.com/ali77gh/ProjectAnalyzer/"; 


pub fn update_handler(){
    webbrowser::open(GITHUB_ADDR).unwrap_or_else(|_|{
        println!("checkout: {}", GITHUB_ADDR)
    });
}