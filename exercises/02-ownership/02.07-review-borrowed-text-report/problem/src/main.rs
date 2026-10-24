// Write a function taking &str that prints its first word and word count.
// Call it twice with the same String, then append to the String after both calls.

fn describe(text: &str) {
    let first_word = text.split_whitespace().next().unwrap_or("");
    let word_count = text.split_whitespace().count();

    println!("First word: {first_word:?}, word count: {word_count}");
}

fn main() {
    let mut text = String::from("Hello mum");

    describe(&text);
    describe(&text);

    text.push_str(" from Rust");
    println!("{text}"); // Hello mum from Rust
}