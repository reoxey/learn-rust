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
    fn test_count_words() {
        assert_eq!(count_words("Hello Rust world"), 3);
        assert_eq!(count_words(""), 0);
        assert_eq!(count_words("  Rust   is\nfast  "), 3);
    }
}