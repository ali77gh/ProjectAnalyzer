use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use tokio::sync::mpsc::Sender;

pub struct Walker {
    root_dir: String,
    ignore: HashSet<PathBuf>,
    output_channel: Sender<PathBuf>,
    postfixes: HashSet<String>,
}

impl Walker {
    pub fn new(
        root_dir: String,
        ignore: &[std::string::String],
        output_channel: Sender<PathBuf>,
        postfixes: Option<HashSet<String>>,
    ) -> Self {
        let mut ignore_hash_set: HashSet<PathBuf> = HashSet::new();

        // passed ignores
        for i in ignore {
            let mut i = i.clone();
            if !i.starts_with("./") {
                i = format!("./{}", i);
            }
            ignore_hash_set.insert(PathBuf::from(i));
        }

        // default ignores
        for i in crate::default_ignore::get_default_ignore() {
            let mut i = i.clone();
            if !i.starts_with("./") {
                i = format!("./{}", i);
            }
            ignore_hash_set.insert(PathBuf::from(i));
        }

        Self {
            root_dir,
            ignore: ignore_hash_set,
            output_channel,
            postfixes: postfixes.expect("TODO auto detect code files"),
        }
    }

    pub fn start(mut self) {
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
                if self.postfixes.contains(postfix) {
                    s.send(x.path()).unwrap();
                }
            };

            std::thread::spawn(move || {
                let _ = Self::walk(
                    Path::new(Path::new(&self.root_dir)),
                    &mut cb,
                    &mut self.ignore,
                );
            });

            while let Ok(path) = r.recv() {
                self.output_channel.send(path).await.unwrap();
            }
        });
    }

    fn walk(
        dir: &Path,
        cb: &mut dyn FnMut(&DirEntry),
        ignore: &mut HashSet<PathBuf>,
    ) -> io::Result<()> {
        if dir.is_dir() {
            for entry in fs::read_dir(dir)? {
                let entry = entry?;
                let path = entry.path();

                if ignore.contains(&path) {
                    ignore.remove(&path);
                    continue;
                }

                if path.is_dir() {
                    Self::walk(&path, cb, ignore)?;
                } else {
                    cb(&entry);
                }
            }
        }
        Ok(())
    }
}
