// Write add_suffix(text: &mut String) that appends a suffix.
// Call it with a mutable String and print the changed value.

fn add_suffix(text: &mut String) {
    text.push_str(", mum!");
}

fn main() {
    let mut text = String::from("Hello");

    add_suffix(&mut text);

    println!("{text}");
}