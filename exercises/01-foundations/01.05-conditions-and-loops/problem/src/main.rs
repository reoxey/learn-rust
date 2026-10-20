fn main() {
    for number in 1..=10 {
        let label = if number % 2 == 0 { "even" } else { "" };
        println!("{number} {label}");
    }

    let mut number = 1;

    let first_above_six = loop {
        if number > 6 {
            break number;
        }
        number += 1;
    };

    println!("First number above 6: {first_above_six}");
}