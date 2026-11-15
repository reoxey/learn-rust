// Implement count_words(text: &str) -> usize.
// Count whitespace-separated words using split_whitespace().
// Keep the function pure and expose it to the binary crate.

pub fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
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