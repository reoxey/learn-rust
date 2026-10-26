// Define a Status enum with Todo, Doing, and Done.
// Write a match expression that produces a readable label for each status.
// Replace Task.done (bool) with Status.
// Update Task::new(), mark_done(), and is_done() to use Status.
// Call all three methods in main().

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
    fn new(title: String) -> Self {
        Self {
            title,
            status: Status::Todo,
        }
    }

    fn mark_done(&mut self) {
        self.status = Status::Done;
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
    let mut task = Task::new(String::from("Learn Rust"));

    println!("Task: {}", task.title);
    println!("Status: {}", task.status_label()); // To Do
    println!("Completed: {}", task.is_done());   // false

    task.status = Status::Doing;
    println!("Status: {}", task.status_label()); // In Progress

    task.mark_done();

    println!("Status: {}", task.status_label()); // Completed
    println!("Completed: {}", task.is_done());   // true
}