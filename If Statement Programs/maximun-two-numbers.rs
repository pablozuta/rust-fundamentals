use std::io;

fn main() {
    println!("Enter first number");
    let mut input1 = String::new();
    io::stdin().read_line(&mut input1).expect("Unable to read number");

    println!("Enter second number");
    let mut input2 = String::new();
    io::stdin().read_line(&mut input2).expect("Unable to read number");

    let number1: i32 = input1.trim().parse().expect("Please enter a valid number");
    let number2: i32 = input2.trim().parse().expect("Please enter a valid number");

    if number1 > number2 {
        println!("{} is greater than {}", number1, number2);
    } else {
        println!("{} is greater than {}", number2, number1);

    }

    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT PROGRAM");
    io::stdin().read_line(&mut input_exit).unwrap();
}