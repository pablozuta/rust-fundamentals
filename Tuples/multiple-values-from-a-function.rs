fn divide_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend & divisor)
}

fn main() {
    let (quotient, remainder) = divide_remainder(10, 3);
    println!("10 divided by 3 is {} with a remainder of {}", quotient, remainder);
}