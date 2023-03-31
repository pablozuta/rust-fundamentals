fn main() {
    let my_array = [2, 5, 44, 32, 89, 11, 3];
    let mut max = 0;
    
    for i in 0..my_array.len() {
        if my_array[i] > max {

            max = my_array[i];
        }
    }

    println!("Max number in array is {}", max);
}