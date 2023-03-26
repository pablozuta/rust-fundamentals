
fn main() {
    let vector = vec! [20, 43, 41, 89, 50];
    let mut largest = vector[0];

    for i in vector {
        if i > largest {
            largest = i;
        }
    }

    println!("Larges Element is: {}", largest);
}