
fn main() {
    let my_array = [1, 2, 1, 5, 6, 5, 6, 5];
    let mut frequency = [0; 10];

    for i in 0..my_array.len() {
        frequency[my_array[i] as usize] += 1;
        }
    for i in 0..frequency.len() {
        if frequency[i] > 0 {
            println!("{} appears {} times", i, frequency[i]);
        }
    }
}