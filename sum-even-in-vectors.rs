
fn main() {
    let vektor = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 22, 54, 88];
    let mut suma = 0;

    for i in vektor {
        if i % 2 == 0 {
            suma += i;
        }
    }

    println!("Sum of even numbers is {}", suma);
}