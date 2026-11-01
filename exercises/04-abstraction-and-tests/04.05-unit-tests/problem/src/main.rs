// Import the temperature conversion function from the library crate.
// Convert 0°C and 100°C to Fahrenheit and print the results.

use temperature_converter::celsius_to_fahrenheit;

fn main() {
    println!("0°C = {}°F", celsius_to_fahrenheit(0.0));
    println!("100°C = {}°F", celsius_to_fahrenheit(100.0));
}