// Parse "42", "bad", and "" as u32.
// Write parse_count(text: &str) -> Result<u32, std::num::ParseIntError>.
// Use match to report success or failure for each input.

fn parse_count(text: &str) -> Result<u32, std::num::ParseIntError> {
    text.parse::<u32>()
}

fn main() {
    let inputs = ["42", "bad", ""];

    for input in inputs {
        match parse_count(input) {
            Ok(count) => println!("Successfully parsed '{input}': {count}"),
            Err(error) => println!("Failed to parse '{input}': {error}"),
        }
    }
}