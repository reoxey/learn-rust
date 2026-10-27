// Build an in-memory Vec of three tasks with different statuses.
// Print each task and count how many have Status::Done.
// Handle a missing index lookup using get() and match.

enum Status {
    Todo,
    Doing,
    Done,
}

struct Task {
    title: String,
    status: Status,
}

impl Task {
    fn new(title: String, status: Status) -> Self {
        Self { title, status }
    }

    fn is_done(&self) -> bool {
        matches!(self.status, Status::Done)
    }

    fn status_label(&self) -> &str {
        match self.status {
            Status::Todo => "To Do",
            Status::Doing => "In Progress",
            Status::Done => "Completed",
        }
    }
}

fn main() {
    let tasks = vec![
        Task::new(String::from("Learn Rust"), Status::Done),
        Task::new(String::from("Practice ownership"), Status::Doing),
        Task::new(String::from("Learn enums"), Status::Todo),
    ];

    let mut done_count = 0;

    // Iterate over borrowed tasks and count completed tasks.
    for task in &tasks {
        println!("{}: {}", task.title, task.status_label());

        if task.is_done() {
            done_count += 1;
        }
    }

    println!("Completed tasks: {}", done_count);

    // Handle a missing index lookup safely.
    let index = 5;

    match tasks.get(index) {
        Some(task) => println!("Found: {}", task.title),
        None => println!("No task found at index {}", index),
    }
}