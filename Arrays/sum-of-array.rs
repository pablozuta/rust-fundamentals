fn main() {
    let my_array = [23, 5, 87, 44, 9, 111];

    let mut sum = 0;
    for i in 0..my_array.len() {
        sum += my_array[i];
    }

    println!("The sum of the array is {}", sum);
}