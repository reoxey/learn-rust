fn main() {
    let number: i32 = 5;
    let decimal: f64 = 3.14;
    let enabled: bool = true;
    let letter: char = 'R';

    let converted: f64 = number as f64;
    let half = converted / 2.0;

    println!("{number}, {decimal}, {enabled}, {letter}");
    println!("Half: {half}");
    println!("5 / 2 = {}", 5 / 2);
    println!("5.0 / 2.0 = {}", 5.0 / 2.0);
}
