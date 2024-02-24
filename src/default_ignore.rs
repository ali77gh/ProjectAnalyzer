use std::{fs, path::Path};

pub(crate) fn get_default_ignore() -> Vec<String> {
    let mut result = vec![];
    if Path::new("./.git").exists() {
        result.push("./.git".to_string());
        println!("ignoring: .git/")
    }
    match fs::read_to_string(Path::new("./.gitignore")) {
        Ok(content) => {
            println!(".gitignore detected (ignoring content)");
            for p in content.split('\n').filter(|p| !p.trim().is_empty()) {
                let mut p = p.to_owned();
                if p.starts_with('/') {
                    p = p.chars().skip(1).collect(); //pop first char
                }
                result.push(p);
            }
        }
        Err(_) => {}
    }

    result
}
