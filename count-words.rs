use std::io;

fn main() {
    println!("Enter a string");
    
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read line");
    
    let count = input.split_whitespace().count();
    
    println!("The number of words is {}", count);

     // wait to press enter to exit
     let mut input_exit = String::new();
     println!("Press Enter to exit...");
     io::stdin().read_line(&mut input_exit).unwrap();

}