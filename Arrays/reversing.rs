fn main() {
    let mut my_array = [1, 2, 3, 4, 5, 6, 7];
    let mut i = 0;
    let mut j = my_array.len() -1;

    while i < j {
        let temp = my_array[i];
        my_array[i] = my_array[j];
        my_array[j] = temp;
        i += 1;
        j -= 1;
          
    }
    println!("{:?}", my_array);
}