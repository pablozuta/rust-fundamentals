
fn main() {
    let numero = 5;
    match numero {
        1 => println!("el numero es 1"),
        2 | 3 => println!("el numero es 2 o 3"),
        4..=7 => println!("el numero esta entre 4 y 7"),
        _ => println!("el numero es algo diferente"),
    }
}