mod task;

use task::{Status, Summary, Task};

fn print_summary(item: &impl Summary) {
    println!("{}", item.summary());
}

fn main() {
    let tasks = vec![
        Task::new(String::from("Learn Rust"), Status::Done),
        Task::new(String::from("Practice ownership"), Status::Doing),
        Task::new(String::from("Learn traits"), Status::Todo),
    ];

    for task in &tasks {
        print_summary(task);
    }
}
