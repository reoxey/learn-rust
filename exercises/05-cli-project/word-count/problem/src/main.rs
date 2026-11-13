// Accept exactly one file path from command-line arguments.
// Read the file using std::fs::read_to_string.
// Count words using the library's count_words() function.
// Report usage errors with exit code 2 and read errors with exit code 1.

use std::env;
use std::fs;
use std::process;

use word_count::count_words;

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

    let word_count = count_words(&content);

    println!("Words: {word_count}");
}
