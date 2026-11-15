// Accept exactly one file path from CLI arguments.
// Print usage and exit with code 2 for invalid arguments.
// Call the library's run() function to count words.
// Print the result on success or exit with code 1 on failure.

use std::env;
use std::process;

use word_count::run;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        process::exit(2);
    }

    let file_path = &args[1];

    match run(file_path) {
        Ok(count) => println!("Words: {count}"),
        Err(error) => {
            eprintln!("Failed to read '{}': {}", file_path, error);
            process::exit(1);
        }
    }
}