// Build a word-count CLI that accepts exactly one file path.
// Use std::env::args() to read command-line arguments.
// Print usage and exit with code 2 for missing or extra arguments.
// Read the file and print its word count.

use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(2);
    }

    let file_path = &args[1];

    let content = fs::read_to_string(file_path).unwrap_or_else(|error| {
        eprintln!("Error reading file: {error}");
        process::exit(1);
    });

    let word_count = content.split_whitespace().count();

    println!("Words: {word_count}");
}