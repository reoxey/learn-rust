// Create a small Vec of tasks.
// Look up tasks by numeric index using tasks.get(index).
// Use match to print the found task or a message when it is missing.
// Try both a valid index and an index beyond the end.

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
        Task::new(String::from("Learn Rust")),
        Task::new(String::from("Practice ownership")),
        Task::new(String::from("Learn enums")),
    ];

    // Try both a valid and an invalid index.
    for index in [1, 2, 5] {
        match tasks.get(index) {
            Some(task) => {
                println!(
                    "Task at index {}: {} ({})",
                    index,
                    task.title,
                    task.status_label()
                );
            }
            None => {
                println!("No task found at index {}", index);
            }
        }
    }
}