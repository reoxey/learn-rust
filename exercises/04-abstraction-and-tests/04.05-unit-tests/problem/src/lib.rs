// Convert Celsius to Fahrenheit using a pure function.
// Test freezing (0°C) and boiling (100°C).

pub fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_freezing() {
        assert_eq!(celsius_to_fahrenheit(0.0), 32.0);
    }

    #[test]
    fn test_boiling() {
        assert_eq!(celsius_to_fahrenheit(100.0), 212.0);
    }
}