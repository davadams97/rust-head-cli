use clap::Parser;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Parser)]
struct Cli {    
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    if !args.path.as_os_str().is_empty() {
        print_lines(args.path)
    }
}

fn print_lines(path: std::path::PathBuf) {
    if let Ok(lines) = read_lines(path) {
        for line in lines.flatten().take(10) {
            println!("{}", line);
        }
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