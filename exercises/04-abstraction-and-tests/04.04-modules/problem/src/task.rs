// Define Task and Status in a separate module.
// Expose only the types and methods required by main.rs using pub.
// Keep internal fields private to enforce encapsulation.

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

    pub fn title(&self) -> &str {
        &self.title
    }

    pub fn is_done(&self) -> bool {
        matches!(self.status, Status::Done)
    }

    pub fn mark_done(&mut self) {
        self.status = Status::Done;
    }

    pub fn status_label(&self) -> &str {
        match self.status {
            Status::Todo => "To Do",
            Status::Doing => "In Progress",
            Status::Done => "Completed",
        }
    }
}