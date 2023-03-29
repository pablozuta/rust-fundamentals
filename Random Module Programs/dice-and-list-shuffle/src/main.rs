use rand::Rng;
use rand::seq::SliceRandom;

fn main() {
    // RANDOM DICE ROLL
    let mut rng = rand::thread_rng();
    let roll = rng.gen_range(1..=6);

    println!("You rolled a {}", roll);

    // RANDOM LIST SHUFFLE
    let mut list = vec![1, 2, 3, 4, 5];
    let mut random_number = rand::thread_rng();

    list.shuffle(&mut random_number);
    println!("Shuffled List: {:?}", list);
 
}
