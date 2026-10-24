// Define Task with title: String and done: bool.
// Construct two tasks and print their fields.

struct Task {
    title: String,
    done: bool,
}

fn main() {
    let task1 = Task {
        title: String::from("Learn Rust"),
        done: false,
    };

    let task2 = Task {
        title: String::from("Read a book"),
        done: true,
    };

    println!("Title: {}, done: {}", task1.title, task1.done);
    println!("Title: {}, done: {}", task2.title, task2.done);
}