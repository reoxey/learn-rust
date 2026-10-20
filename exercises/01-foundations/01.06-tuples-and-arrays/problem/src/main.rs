fn main() {
    let person: (&str, u32) = ("Alice", 30);
    let (name, age) = person;

    println!("Name: {name}, age: {age}");

    let numbers = [10, 20, 30, 40, 50];
    let mut sum = 0;

    for number in numbers {
        sum += number;
    }

    println!("Sum: {sum}");
}