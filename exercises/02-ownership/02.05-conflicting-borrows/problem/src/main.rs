// Borrow a String immutably, then try to mutate it before using that reference again.
// Read the error. Repair it by making the last use of the shared borrow occur before mutation.

// error:
// fn main() {
//     let mut text = String::from("Hello");
//     let borrowed = &text;
//
//     text.push_str(", mum!"); // Error E0502: cannot borrow `text` as mutable
//                               // because it is also borrowed as immutable.
//
//     println!("{borrowed}");    // Immutable borrow is still used here.
// }

// fixed:
fn main() {
    let mut text = String::from("Hello");
    let borrowed = &text;

    println!("{borrowed}"); // Last use of the immutable borrow.

    text.push_str(", mum!"); // Mutable borrow is now allowed.
    println!("{text}");
}
