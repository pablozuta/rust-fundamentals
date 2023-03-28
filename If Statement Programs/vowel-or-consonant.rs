use std::io;

fn main() {
    println!("Enter a character:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");

    let character = input.trim().chars().next().expect("Please enter a valid character");

    if character == 'a' || character == 'e' || character == 'i' || character == 'o' || character == 'u'  {
        println!("{} is a vowel", character);
    } else {
        println!("{} is a consonant", character);

    }

    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT");
    io::stdin().read_line(&mut input_exit).unwrap();
}