use std::fs::File;
use std::io::Write;

fn main() {
    let my_string = "Y se oia como el viento llevaba y traia aquel rumor, revolviendolo, hasta hacer de el un solo mugido.";

    let mut file = File::create("output.txt").expect("No se pudo crear el archivo");
    file.write_all(my_string.as_bytes()).expect("No se pudo escribir en el archivo");
}