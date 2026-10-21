// Write a function that takes a String and returns it after appending
// an exclamation mark. Call it and bind its returned value.

fn add_exclamation(mut text: String) -> String {
    text.push('!');
    text
}

fn main() {
    let original = String::from("Hello mum");
    let result = add_exclamation(original);

    println!("{result}");
}