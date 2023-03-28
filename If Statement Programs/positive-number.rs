use std::io;

fn main() {
    // pregunta ingresar un numero y lo guarda en una variable
    println!("Enter a number:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
    .expect("Failed to read number"); // metodo to handle errors

    // se transforma el input que viene en formato string a i32
    let number: i32 = input.trim().parse()
    .expect("Please enter a valid number"); // handle error si lo ingresado no es un numero
    
    // metodo if para mostrar mensajes dependiendo si el numero es mayor , menor o igual a 0
    if number > 0 {
        println!("{} is a positive number", number);
    } else if number < 0 {
        println!("{} is a negative number", number);
    }else {
        println!("number is zero");
    }

    // wait to press enter to exit
    let mut input_exit = String::new();
    println!("Press Enter to exit...");
    io::stdin().read_line(&mut input_exit).unwrap();
}