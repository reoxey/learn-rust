// Write double_count(text: &str) -> Result<u32, std::num::ParseIntError>.
// Parse the input using the ? operator and double the result.
// Restrict successful test inputs to values between 0 and 100.
// Use match to handle both successful and failed parsing.

fn double_count(text: &str) -> Result<u32, std::num::ParseIntError> {
    let count = text.parse::<u32>()?;
    Ok(count * 2)
}

fn main() {
    let inputs = ["0", "25", "42", "100", "bad", ""];

    for input in inputs {
        match double_count(input) {
            Ok(result) => println!("Input: '{input}', Doubled: {result}"),
            Err(error) => println!("Input: '{input}', Error: {error}"),
        }
    }
}
