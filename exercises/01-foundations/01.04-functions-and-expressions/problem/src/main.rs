fn c_to_f(c: f64) -> f64 {
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    println!("{}", c_to_f(100.0));
}
