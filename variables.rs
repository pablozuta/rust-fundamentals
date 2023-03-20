fn main() {
    let numero = 2666;
    let frase = "Rust Programming Language";
    let f: f64 = 3.14;
    
    
    // mutable values
    let mut numero_mutable = 20;
    
    
    println!("{}", numero);
    println!("{}", frase);
    println!("{}", f);
    println!("{}", numero_mutable);
    numero_mutable = 421;
    println!("{}", numero_mutable);
    
    // booleans
    let is_rust_fun = true;
    if is_rust_fun {
        println!("Rust is Fun");        
    } else {
        println!("Rust is NOT Fun");        

    }
}
