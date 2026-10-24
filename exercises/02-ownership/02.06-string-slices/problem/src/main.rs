// Write first_word(text: &str) -> &str using split_whitespace().next().unwrap_or("").
// Try a literal, a borrowed String, empty input, and leading spaces.

fn first_word(text: &str) -> &str {
    text.split_whitespace().next().unwrap_or("")
}

fn main() {
    println!("{}", first_word("Hello mum!")); // Hello

    let text = String::from("Rust is fun");
    println!("{}", first_word(&text)); // Rust

    println!("{:?}", first_word("")); // ""
    println!("{}", first_word("   Leading spaces")); // Leading
}
