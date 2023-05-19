fn main() {
    // predefined character names from the Akira anime
    let character_names = vec![
        "Kaneda",
        "Tetsuo",
        "Akira",
        "Kei",
        "Ryu",
        "Colonel",
        "Yamagata",
    ];
    // for loop para mostrar todos los nombres
    for character in character_names {
        println!("{}", character);
    }
    

    // Input character name
    let input_name = "Kaneda";

    // match the input name against predefined names
    match input_name {
        "Kaneda" => println!("Input name matches Kaneda"),
        "Tetsuo" => println!("Input name matches Tetsuo"),
        "Akira" => println!("Input name matches Akira"),
        "Kei" => println!("Input name matches Kei"),
        "Ryu" => println!("Input name matches Ryu"),
        "Colonel" => println!("Input name matches Colonel"),
        "Yamagata" => println!("Input name matches Yamagata"),
        _ => println!("Input name do not match any character"),
    }


}