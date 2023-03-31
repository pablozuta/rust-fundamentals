fn main() {
    let mut my_array = [[0; 3]; 2];
    my_array[0][0] = 1;
    my_array[0][1] = 2;
    my_array[0][2] = 3;
    my_array[1][0] = 4;
    my_array[1][1] = 5;
    my_array[1][2] = 6;

    for i in 0..2 {
        for j in 0..3 {
            println!("My array [{}][{}] = {}", i, j, my_array[i][j]);
        }
    }
}