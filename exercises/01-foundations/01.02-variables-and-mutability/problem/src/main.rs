fn main() {
    println!("Hello, mum!");

    let immutable = 10;

    println!("Immutable: {}", immutable);

    // error[E0384]: cannot assign twice to immutable variable `immutable`
    // immutable = 20;

    let mut mutable = 20;

    mutable += 2;

    println!("Mutable: {}", mutable);
}
