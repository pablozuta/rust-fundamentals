use std::io;

fn main() {
    println!("Enter a number");
    let mut input = String::new();

    io::stdin().read_line(&mut input).expect("Failed to read line");

    let n: i32 = input.trim().parse().expect("Please enter a valid integer");

    let mut a = 0;
    let mut b = 1;
    let mut sum;

    println!("Fibonacci sequence up to {}", n);

    while a <= n {
        println!("{}", a);

        sum = a + b;
        a = b;
        b = sum;
    }

    // wait to press enter to exit
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input).unwrap();

    

}