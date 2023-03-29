use std::io;

fn main() {
    println!("Enter temperature in Fahrenheit:");

    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Error Reading Input");

    let fahrenheit: f32 = input.trim().parse()
    .expect("Please enter a valid number");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;

    println!("Temperature in celsius is {:.2}", celsius);

    // WAIT FOR EXIT
    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT");
    io::stdin().read_line(&mut input_exit).unwrap();
}