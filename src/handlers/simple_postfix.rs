use std::collections::{HashMap, HashSet};
use std::{path::Path, fs::DirEntry, sync::mpsc, thread};
use crate::utils::walker::walk;
use std::fs;
use std::path::PathBuf;
use piechart::{Chart, Color, Data, Style};

pub fn simple_postfix_handler(postfixes: Vec<String>){
    let mut file_counter: HashMap<String, u64> = HashMap::new();
    let mut line_counter: HashMap<String, u64> = HashMap::new();
    let mut post_set: HashSet<String> = HashSet::new();

    for p in postfixes{
        file_counter.insert(p.clone(), 0);
        line_counter.insert(p.clone(), 0);
        post_set.insert(p.clone());
    }

    let (tx12, rx12) = mpsc::channel::<Option<PathBuf>>(); // channel thread 1,2
    // thread1: walk and filter by postfix
    let post_set_t1 = post_set.clone();
    thread::spawn(move ||{
        let mut cb = |x:&DirEntry|{
            let binding = x.file_name();
            let postfix = binding.to_str().unwrap_or("").split(".").last().unwrap_or("nothing_file");
            if post_set_t1.contains(postfix){
                tx12.send(Some(x.path())).unwrap();
            }
        };
        let _ = walk(Path::new("./"), &mut cb);
        tx12.send(None).unwrap();
    });

    let (tx23, rx23) = mpsc::channel::<Option<FileWithPost>>();  // channel thread 2,3
    // thread2: file reader
    thread::spawn(move || {
        while let Some(x) = rx12.recv().unwrap(){
            let postfix = x.file_name().unwrap().to_str().unwrap_or("").split(".").last().unwrap_or("nothing_file");
            let content = fs::read(x.clone())
                .unwrap_or_else(|_|{ 
                    println!("can't read file:{}", x.to_str().unwrap()); vec![]
            });
            tx23.send(Some(FileWithPost::new(postfix.to_string(), content))).unwrap();
        }
        tx23.send(None).unwrap();
    });

    // main thread: counting
    let nl = b'\n';
    let nl = &&nl; // pre process dereference
    while let Some(x) = rx23.recv().unwrap(){
        let fc = file_counter.get_mut(x.postfix.as_str()).unwrap();
        let lc = line_counter.get_mut(x.postfix.as_str()).unwrap();
        *fc += 1;
        *lc += x.bytes.iter().filter(|byte|byte==nl).count() as u64;
    }


    println!("┌───────────────────────────────────────────────┐");
    println!("│                ProjectAnalyzer                │");
    println!("│                                               │");
    println!("│ https://github.com/ali77gh/ProjectAnalyzer    │");
    println!("│                                               │");
    for postfix in post_set.iter(){
        let file_counter = file_counter.get(postfix).unwrap();
        let line_counter = line_counter.get(postfix).unwrap();
        let division = if file_counter==&0 {0} else { line_counter / file_counter };
        println!("├───────────────────────────────────────────────┤");
        println!("│ {} files result:                 \t\t│", postfix);
        println!("│   {} {} files                   \t\t│", file_counter, postfix);
        println!("│   {} lines of {}               \t\t│", line_counter, postfix);
        println!("│   average lines per file: {}       \t\t│", division);
    }
    println!("└───────────────────────────────────────────────┘");
    
    let chars = vec!['▰', '▼', '◆', '■', '•', '▪', '▴','▰', '▼', '◆', '■', '•', '▪', '▴','▰', '▼', '◆', '■', '•', '▪', '▴','▰', '▼', '◆', '■', '•', '▪', '▴', ];
    let colors:Vec<Style> = vec![
        Color::Green.into(),Color::Yellow.into(),Color::Blue.into(),Color::Purple.into(),
        Color::Cyan.into(),Color::White.into(),Color::Red.into(),
        Color::Green.into(),Color::Yellow.into(),Color::Blue.into(),Color::Purple.into(),
        Color::Cyan.into(),Color::White.into(),Color::Red.into(),
        Color::Green.into(),Color::Yellow.into(),Color::Blue.into(),Color::Purple.into(),
    ];
    let mut dataset = vec![];
    let mut counter = 0;
    for lang in line_counter{
        dataset.push(Data { 
            label: lang.0.into(),
            value: lang.1 as f32,
            color: Some(*colors.get(counter).unwrap()),
            fill: *chars.get(counter).unwrap() 
        });
        counter += 1;
    }
    // draw chart
    Chart::new()
        .radius(counter as u16 +4)
        .aspect_ratio(3)
        .legend(true)
        .draw(&dataset);

}

struct FileWithPost{
    postfix: String,
    bytes: Vec<u8>
}

impl FileWithPost{
    
    fn new(postfix: String, bytes:Vec<u8>) -> Self{
        FileWithPost{ postfix, bytes }
    }
}