// Create a String and assign it to another variable.
// Try to print the original and record the error.
// Repair the program by printing the new owner.

fn main() {
    let original = String::from("Hello");
    let new_owner = original;

    // println!("{original}");
    // >> error[E0382]: borrow of moved value: `original`

    println!("{new_owner}"); // Hello
}