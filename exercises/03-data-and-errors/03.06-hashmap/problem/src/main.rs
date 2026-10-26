// Count occurrences of words in a short string using HashMap<String, usize>.
// Use entry(...).or_insert(0) to initialize missing words with a count of 0.
// Print each word and its frequency.

use std::collections::HashMap;

fn main() {
    let text = "rust is fast and rust is safe";

    let mut word_counts: HashMap<String, usize> = HashMap::new();

    for word in text.split_whitespace() {
        let count = word_counts.entry(word.to_string()).or_insert(0);
        *count += 1;
    }

    for (word, count) in &word_counts {
        println!("{word}: {count}");
    }
}