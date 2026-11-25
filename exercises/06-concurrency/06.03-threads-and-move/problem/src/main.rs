// Spawn a thread that takes ownership of a Vec<i32> using move.
// Calculate and print the sum inside the spawned thread.
// Join the thread before main returns.
// Handle a failed join with a readable error message.

use std::thread;

fn main() {
    let numbers = vec![10, 20, 30, 40, 50];

    let handle = thread::spawn(move || {
        let sum: i32 = numbers.iter().sum();
        println!("Sum from thread: {sum}");
    });

    match handle.join() {
        Ok(()) => println!("Thread completed successfully"),
        Err(error) => {
            eprintln!("Thread panicked: {error:?}");
        }
    }
}