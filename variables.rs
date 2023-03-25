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

    // EXPLICIT DECLARATION
    let num_int: i32 = 31;
    let num_int2: u64 = 483;
    println!("La suma de {} y {} es {}", num_int, num_int2, num_int as u64 + num_int2);
    
    // EXPLICIT FLOATS
    let num_float: f32 = 3.13;
    let num_float2: f64 = 54.24212;
    println!("La multiplicacion de {} y {} es {}", num_float, num_float2, num_float as f64 * num_float2);

    // CHARS
    let caracter: char = '🚀';
    println!("El caracter es {}", caracter);
}
