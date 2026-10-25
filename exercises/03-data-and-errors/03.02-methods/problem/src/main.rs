// Create a Task struct with a title and completion status.
// Add Task::new(title: String), mark_done(&mut self), and is_done(&self).
// Call all three methods in main().

struct Task {
    title: String,
    done: bool,
}

impl Task {
    fn new(title: String) -> Self {
        Self {
            title,
            done: false,
        }
    }

    fn mark_done(&mut self) {
        self.done = true;
    }

    fn is_done(&self) -> bool {
        self.done
    }
}

fn main() {
    let mut task = Task::new(String::from("Learn Rust"));

    println!("Task: {}", task.title);
    println!("Completed: {}", task.is_done()); // false

    task.mark_done();

    println!("Completed: {}", task.is_done()); // true
}