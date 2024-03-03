# ProjectAnalyzer

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/) <br>


Fast Rust binary that counts line numbers of a codebase.

<img src="./screen_shot.png" width=400> <br>


## How to use

```posh
Fast Rust binary that counts line numbers of a codebase

Usage: project_analyzer [OPTIONS] [COMMAND]

Commands:
  update  Opens github
  help    Print this message or the help of the given subcommand(s)

Options:
  -r, --root-dir <ROOT_DIR>    Number of times to greet [default: .]
      --ignore <IGNORE>        Example: project_analyzer --ignore node_modules --ignore dist
  -w, --watch                  Will keep running and update result whenever anything changed
  -p, --postfixes <POSTFIXES>  Filter by list of file postfixes example: project_analyzer --postfixes py,rs,cpp
  -j, --json                   Output as json
  -h, --help                   Print help
  -V, --version                Print version
```


## Installation

### 1. Download binary:
[Release page](https://github.com/ali77gh/ProjectAnalyzer/releases)\
or  build it yourself:

```sh
cd /tmp 
git clone git@github.com:ali77gh/ProjectAnalyzer.git
cd ProjectAnalyzer
cargo build --release
```

### 2. Add binary to your PATH

```sh
# linux
cp ./target/release/project_analyzer /usr/bin

# MacOS
cp ./target/release/project_analyzer /usr/local/bin/

# Windows
# add binary to Environment Variables Path
```

TODOs:

- [ ] VS-Code extension
- [ ] Git history for more analyze
- [ ] a website to do all this in frontend by downloading zip from github + WASM (is that even possible? will CORS block me to do this?)
