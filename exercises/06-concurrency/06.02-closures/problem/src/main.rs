// Create a closure that filters numbers above a captured threshold.
// Change the threshold before defining a second closure.
// Compare the results of both closures.

fn main() {
    let numbers = vec![5, 10, 15, 20, 25, 30];

    let mut threshold = 10;

    // Capture the current threshold by value.
    let filter1 = {
        let captured = threshold;
        move |n: &&i32| **n > captured
    };

    // Change the threshold before defining another closure.
    threshold = 20;

    let filter2 = |n: &&i32| **n > threshold;

    let result1: Vec<i32> = numbers.iter().filter(filter1).copied().collect();
    let result2: Vec<i32> = numbers.iter().filter(filter2).copied().collect();

    println!("Threshold 10: {:?}", result1);
    println!("Threshold 20: {:?}", result2);
}