use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead, stdin};
use std::path::Path;

#[derive(Parser)]
struct Cli {    
    path: Option<std::path::PathBuf>,
}

fn main() {
    let args = Cli::parse();

    if let Some(path) = args.path {
        print_lines(path);
    } else {
        start_eval();
    }
}

fn print_lines(path: std::path::PathBuf) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten().take(10) {
            println!("{}", line);
        }
    }
}

fn start_eval() {
    for _ in 0..10 {
        
        let mut input = String::new();
        stdin().read_line(&mut input).unwrap();
        let command = input.trim();

        println!("{}", command);
    }
}

// Used from - https://doc.rust-lang.org/rust-by-example/std_misc/file/read_lines.html?search=where
// returns an io::Result containing an iterator over lines (io::Lines) wrapped in a buffered reader (io::BufReader) for a file (File)
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    
    // Returns an Iterator to the Reader of the lines of the file.
    Ok(io::BufReader::new(file).lines())
}