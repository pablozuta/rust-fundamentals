// FizzBuzz is a classic programming exercise 
// where you print out the numbers from 1 to 100
// replacing multiples of 3 with "Fizz", multiples of 5 with "Buzz"
// and multiples of both with "FizzBuzz". 

fn main() {
    for i in 1..=100{
        match(i % 3, i % 5) {
            (0, 0) => println!("FizzBuzz"),
            (0, _) => println!("Fizz"),
            (_, 0) => println!("Buzz"),
            _ => println!("{}", i),
        }
        
    }
}