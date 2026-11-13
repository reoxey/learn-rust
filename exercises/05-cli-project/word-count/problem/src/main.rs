// Build a word-count CLI that accepts exactly one file path.
// Read the file using std::fs::read_to_string.
// Report read failures to stderr and exit with code 1.
// Report invalid arguments and exit with code 2.
// Count and print the whitespace-separated words.

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
        eprintln!("Failed to read '{}': {}", file_path, error);
        process::exit(1);
    });

    let word_count = content.split_whitespace().count();

    println!("Words: {word_count}");
}