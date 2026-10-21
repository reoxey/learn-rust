// Write describe(text: &str) that prints text without taking ownership.
// Call it with a borrowed String, then print the original String.

fn describe(text: &str) {
    println!("{text}");
}

fn main() {
    let original = String::from("Hello");

    describe(&original); // &String automatically coerces to &str

    println!("{original}"); // Still valid: ownership wasn't moved
}