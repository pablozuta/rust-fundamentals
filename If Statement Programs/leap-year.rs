use std::io;

fn main() {
    println!("Enter a year:");
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Unable to read line");

    let year: i32 = input.trim().parse().expect("Please enter a valid year");

    let is_leap_year = if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        true
    } else {
        false
    };

    if is_leap_year {
        println!("{} year is leap", year);
    }else {
        println!("{} year is NOT leap", year);
    }

    let mut input_exit = String::new();
    println!("PRESS ENTER TO EXIT");
    io::stdin().read_line(&mut input_exit).unwrap();
}