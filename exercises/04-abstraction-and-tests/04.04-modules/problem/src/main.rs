// Import Task and Status from the task module.
// Create three tasks, print their statuses, and count completed tasks.
// Handle a missing index lookup safely.

mod task;

use task::{Status, Task};

fn main() {
    let mut tasks = vec![
        Task::new(String::from("Learn Rust"), Status::Done),
        Task::new(String::from("Practice ownership"), Status::Doing),
        Task::new(String::from("Learn enums"), Status::Todo),
    ];

    // Modify a task using its public method.
    if let Some(task) = tasks.get_mut(2) {
        task.mark_done();
    }

    // Print each task using public methods.
    for task in &tasks {
        println!("{}: {}", task.title(), task.status_label());
    }

    // Count completed tasks.
    let done_count = tasks.iter().filter(|task| task.is_done()).count();
    println!("Completed tasks: {done_count}");

    // Handle a missing index.
    match tasks.get(5) {
        Some(task) => println!("Found: {}", task.title()),
        None => println!("Task not found at index 5"),
    }
}