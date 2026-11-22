// Filter positive numbers from a Vec<i32>, double them,
// and collect the results into a new Vec.
// Use iter() to borrow elements without consuming the original vector.

fn main() {
    let numbers: Vec<i32> = vec![-3, 5, 0, 8, -2, 10];

    let doubled: Vec<i32> = numbers
        .iter()
        .filter(|&&n| n > 0)
        .map(|&n| n * 2)
        .collect();

    println!("Original: {:?}", numbers);
    println!("Doubled: {:?}", doubled);
}