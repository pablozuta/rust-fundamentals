
fn main() {
    let frase = "El rio comenzo a crecer hace tres noches, a eso de la madrugada";
    println!("{}", frase);
    
    // obtener la longitud de un string
    let len = frase.len();
    println!("{}", len);
    
    // to_uppercase method
    println!("{}", frase.to_uppercase());
    
    // to_lowercase method
    println!("{}", frase.to_lowercase());
    
    // triming el principio y final
    let frase_para_trim = "        Entre Ser y Hacer           " ;
    println!("{}", frase_para_trim.trim());


}