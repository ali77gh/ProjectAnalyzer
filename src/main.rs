use std::{
    path::Path, 
    io,
    fs::{self, DirEntry},
    env
};

fn visit_dirs(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                visit_dirs(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}

fn main() {
    
    let args: Vec<String> = env::args().collect();
    let prefix = args.get(1);

    if let Some(p) = prefix{
        let _ = visit_dirs(Path::new("./"), &|x|{
            if x.file_name().to_str().unwrap().ends_with(p){
                println!("{:?}", x.path());
            }
        });
    }else{
        println!("TODO: show help")
    }

}
