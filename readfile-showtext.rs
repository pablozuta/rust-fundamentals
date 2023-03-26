use std::fs;

fn main() {
    // leer el contenido del archivo y guardarlo en una variable
    let file_content = fs::read_to_string("output.txt")
    .expect("No se pudo leer el archivo");

    // mostrar el contenido por terminal
    println!("{}", file_content);
}