use std::io;

fn main() {
    println!("Please enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");

    let number: i32 = input.trim().parse().expect("Please enter a valid number");

    if number % 2 == 0 {
        println!("{} is even", number);
    } else {
        println!("{} is odd", number);
    }

    // WAIT TO EXIT
    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT");
    io::stdin().read_line(&mut input_exit).unwrap();


}