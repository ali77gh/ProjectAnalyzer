use std::{
    collections::HashSet,
    fs::{self, DirEntry},
    io,
    path::{Path, PathBuf},
};

use tokio::sync::mpsc::Sender;

use crate::arg_parser::MyArgs;

pub struct Walker {
    root_dir: String,
    ignore: HashSet<PathBuf>,
    output_channel: Sender<PathBuf>,
    postfixes: Option<HashSet<String>>,
}

impl Walker {
    pub fn new(args: &MyArgs, output_channel: Sender<PathBuf>) -> Self {
        let mut ignore_hash_set: HashSet<PathBuf> = HashSet::new();

        // passed ignores
        for i in args.ignore() {
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

        let postfixes = match args.postfixes() {
            Some(arg_p) => {
                let mut postfixes = HashSet::<String>::new();
                for p in arg_p {
                    postfixes.insert(p);
                }
                Some(postfixes)
            }
            None => None,
        };

        Self {
            root_dir: args.root_dir().to_string(),
            ignore: ignore_hash_set,
            output_channel,
            postfixes,
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
                    .split('.')
                    .last()
                    .unwrap_or("nothing_file");

                match &self.postfixes {
                    Some(p) => {
                        if p.contains(postfix) {
                            s.send(x.path()).unwrap();
                        }
                    }
                    None => {
                        use crate::default_postfixes::DEFAULT_POSTFIXES;
                        if DEFAULT_POSTFIXES.contains(postfix) {
                            s.send(x.path()).unwrap();
                        }
                    }
                };
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
