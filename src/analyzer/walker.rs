use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use tokio::sync::mpsc::Sender;

pub struct Walker {
    root_dir: String,
    output_channel: Sender<PathBuf>,
    post_fixes: HashSet<String>,
}

impl Walker {
    pub fn new(
        root_dir: String,
        output_channel: Sender<PathBuf>,
        post_fixes: Option<HashSet<String>>,
    ) -> Self {
        Self {
            root_dir,
            output_channel,
            post_fixes: post_fixes.expect("TODO auto detect code files"),
        }
    }

    pub fn start(self) {
        tokio::spawn(async move {
            let (s, r) = std::sync::mpsc::channel();
            let mut cb = move |x: &DirEntry| {
                let binding = x.file_name();
                let postfix = binding
                    .to_str()
                    .unwrap_or("")
                    .split(".")
                    .last()
                    .unwrap_or("nothing_file");
                if self.post_fixes.contains(postfix) {
                    s.send(x.path()).unwrap();
                }
            };

            std::thread::spawn(move || {
                let _ = Self::walk(Path::new("./"), &mut cb);
            });

            while let Ok(path) = r.recv() {
                self.output_channel.send(path).await.unwrap();
            }
        });
    }

    fn walk(dir: &Path, cb: &mut dyn FnMut(&DirEntry)) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();
                if path.is_dir() {
                    Self::walk(&path, cb)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}
