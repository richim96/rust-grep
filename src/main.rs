use std::io::BufReader;
use std::io::prelude::*;
use std::fs::File;
use std::path::PathBuf;

use anyhow::Result;
use clap::Parser;

// Set custuom error handler
#[derive(Debug)]
struct CustomError(String);

// Set cli parser
#[derive(Parser)]
struct Cli {
    pattern: String,
    file_path: std::path::PathBuf,
}

fn main() -> Result<()> {
    // Parse cli arguments
    let args = Cli::parse();
    let file_path: PathBuf = args.file_path;
    let pattern: String = args.pattern;

    // Open file and create a buffer reader to avoid large in-memory reads
    let file: File = match File::open(&file_path) {
        Ok(file) => { file }
        Err(error) => { panic!("\n! Error reading '{}'\n    {}\n", file_path.display(), error)}
    };
    let buf_reader: BufReader<File> = BufReader::new(file);

    // Iterate lines and find matching pattern
    println!("\nMATCHING LINES:");
    for (line_number, reader_result) in buf_reader.lines().enumerate() {
        let line: String = reader_result.unwrap();

        if line.contains(&pattern) {
            println!("({}) {}", line_number, line.trim());
        }
    }
    println!();

    Ok(())
}
