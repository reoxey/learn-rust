// Create a Vec<Task> and push two tasks into it.
// Use get_mut() to modify one task.
// Iterate over borrowed tasks to print their titles and statuses.

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

    fn status_label(&self) -> &str {
        match self.status {
            Status::Todo => "To Do",
            Status::Doing => "In Progress",
            Status::Done => "Completed",
        }
    }
}

fn main() {
    let mut tasks: Vec<Task> = Vec::new();

    tasks.push(Task::new(String::from("Learn Rust")));
    tasks.push(Task::new(String::from("Practice ownership")));

    // get_mut() returns Option<&mut Task>
    if let Some(task) = tasks.get_mut(0) {
        task.mark_done();
    }

    // Borrow tasks without transferring ownership
    for task in &tasks {
        println!("{}: {}", task.title, task.status_label());
    }
}