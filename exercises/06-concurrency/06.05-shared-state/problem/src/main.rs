// Use Arc<Mutex<u32>> to share a counter between two threads.
// Each thread increments the counter exactly once.
// Keep the lock scope short and join both threads.
// Print the final counter value.

use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    let counter = Arc::new(Mutex::new(0u32));

    let mut handles = Vec::new();

    for _ in 0..2 {
        let counter = Arc::clone(&counter);

        let handle = thread::spawn(move || {
            {
                let mut count = counter.lock().unwrap();
                *count += 1;
            } // MutexGuard is dropped here, releasing the lock.
        });

        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    println!("Final counter: {}", *counter.lock().unwrap());
}