use std::{
    path::Path, 
    io,
    fs::{self, DirEntry}
};

pub fn walk(dir: &Path, cb: &dyn Fn(&DirEntry)) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_dir() {
                walk(&path, cb)?;
            } else {
                cb(&entry);
            }
        }
    }
    Ok(())
}