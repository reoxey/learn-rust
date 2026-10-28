// Write first_task<'a>(tasks: &'a [Task], title: &str) -> Option<&'a Task>.
// Search for the first task whose title matches the given title.
// Return a reference to the matching task, or None if not found.
// Explain why the returned reference borrows from tasks, not title.

struct Task {
    title: String,
}

impl Task {
    fn new(title: String) -> Self {
        Self { title }
    }
}

fn first_task<'a>(tasks: &'a [Task], title: &str) -> Option<&'a Task> {
    tasks.iter().find(|task| task.title == title)
}

fn main() {
    let tasks = vec![
        Task::new(String::from("Learn Rust")),
        Task::new(String::from("Practice ownership")),
        Task::new(String::from("Learn lifetimes")),
    ];

    let title = String::from("Practice ownership");

    match first_task(&tasks, &title) {
        Some(task) => println!("Found task: {}", task.title),
        None => println!("Task not found"),
    }

    match first_task(&tasks, "Learn Go") {
        Some(task) => println!("Found task: {}", task.title),
        None => println!("Task not found"),
    }
}