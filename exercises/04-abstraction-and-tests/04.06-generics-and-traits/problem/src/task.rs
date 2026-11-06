// Define a Summary trait with summary(&self) -> String.
// Implement Summary for Task.
// Write print_summary(item: &impl Summary) to print any type
// that implements the Summary trait.

pub trait Summary {
    fn summary(&self) -> String;
}

pub enum Status {
    Todo,
    Doing,
    Done,
}

pub struct Task {
    title: String,
    status: Status,
}

impl Task {
    pub fn new(title: String, status: Status) -> Self {
        Self { title, status }
    }

    pub fn status_label(&self) -> &str {
        match self.status {
            Status::Todo => "To Do",
            Status::Doing => "In Progress",
            Status::Done => "Completed",
        }
    }
}

// Implement the Summary trait for Task.
impl Summary for Task {
    fn summary(&self) -> String {
        format!("{}: {}", self.title, self.status_label())
    }
}
