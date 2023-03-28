use std::io;

fn main() {
    println!("Enter the student's score (between 10 and 100):");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read input");

    let score: i32 = input.trim().parse().expect("PLease enter a valid number");

    let grade = if score >= 90 {
        "A"
    } else if score >= 80 {
        "B"
    } else if score >= 70 {
        "C"
    } else if score >= 60 {
        "D"
    } else {
        "F"
    };
    
    println!("The student grade is a {}", grade);

    // WAIT TO EXIT
    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT");
    io::stdin().read_line(&mut input_exit).unwrap();
}