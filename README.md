# ProjectAnalyzer

[![forthebadge made-with-rust](http://ForTheBadge.com/images/badges/made-with-rust.svg)](https://www.rust-lang.org/) <br>


Fast Rust binary that counts line numbers of a project + a nice pie chart using [piechar](https://github.com/jakobhellermann/piechart).

<img src="./screen_shot.png" width=400> <br>

## Performance

Project analyzer can count c, h, rs, asm files of linux kernel in 900ms on my Laptop which is 6-7 times faster than old [python implementation](https://github.com/ali77gh/ProjectAnalyzer/tree/python-impl).

```posh
Thread_1    ➡️    Thread_2    ️️➡️    Thread_3
(filter files)   (read files)      (count newlines)
```

<br>

## How to use

```posh
┌───────────────────────────────────────────────┐
│                ProjectAnalyzer                │
│                                               │
│ https://github.com/ali77gh/ProjectAnalyzer    │
│                                               │
│ How to use:                                   │
│  $ project_analyzer <postfixes>               │
│                                               │
│ Single language Example:                      │
│  $ project_analyzer py                        │
│                                               │
│ Multi language Example:                       │
│  $ project_analyzer py,rs,h,c                 │
│                                               │
│ Options:                                      │
│  -h, --help                                   │
│  -v, --version                                │
│  -u, --update                                 │
│                                               │
└───────────────────────────────────────────────┘
```

<br>

## Installaion

```sh
cd /tmp 
git clone git@github.com:ali77gh/ProjectAnalyzer.git
cd ProjectAnalyzer
cargo build --release

# linux
cp ./target/release/project_analyzer /usr/bin

# MacOS
cp ./target/release/project_analyzer /usr/local/bin/

# Windows
# add binary to Environment Variables Path
```
