// Spawn a worker thread that sends three integers through an mpsc channel.
// Receive and print the integers in the main thread.
// Let the worker automatically drop its sender when finished.
// Join the worker thread and handle a possible panic.

use std::sync::mpsc;
use std::thread;

fn main() {
    let (tx, rx) = mpsc::channel::<i32>();

    let handle = thread::spawn(move || {
        for number in [10, 20, 30] {
            tx.send(number).expect("Failed to send number");
        }
        // tx is automatically dropped when the worker finishes.
    });

    // Receive messages until the channel closes.
    for number in rx {
        println!("Received: {number}");
    }

    match handle.join() {
        Ok(()) => println!("Worker completed successfully"),
        Err(_) => eprintln!("Worker thread panicked"),
    }
}