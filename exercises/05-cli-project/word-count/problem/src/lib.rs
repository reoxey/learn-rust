// Extract run(path: &str) -> Result<usize, std::io::Error>.
// Read the requested file and return its word count.
// Propagate file-reading errors using the ? operator.
// Keep word counting separate from file I/O.

use std::fs;
use std::io;

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

pub fn run(path: &str) -> Result<usize, io::Error> {
    let content = fs::read_to_string(path)?;
    Ok(count_words(&content))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_content() {
        assert_eq!(count_words(""), 0);
    }

    #[test]
    fn test_whitespace_only() {
        assert_eq!(count_words("   \t\n  "), 0);
    }

    #[test]
    fn test_repeated_spaces() {
        assert_eq!(count_words("Rust   is    fast"), 3);
    }

    #[test]
    fn test_multiple_lines() {
        let text = "Rust is fast\nRust is safe\nLearning Rust";
        assert_eq!(count_words(text), 8);
    }
}