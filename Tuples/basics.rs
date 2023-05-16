fn main() {
    // creating a tuple
    let my_tuple = (1, 20.5, "Hello Rust");

    // accesing tuple elements
    println!("{}", my_tuple.0);
    println!("{}", my_tuple.1);
    println!("{}", my_tuple.2);

    // destructuring a tuple
    let (variable_uno, variable_dos, variable_tres) = my_tuple;
    println!("Destructured values:");
    println!("{}", variable_uno);
    println!("{}", variable_dos);
    println!("{}", variable_tres);

    // tuple with different types
    let mixed_tuple = (10, "Pedro Paramo", true);
    let (x, y, z) = mixed_tuple;
    println!("Mixed Tuple :{}, {}, {}", x, y, z);
}