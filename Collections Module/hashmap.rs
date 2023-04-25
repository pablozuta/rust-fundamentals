// This program creates a HashMap, inserts some key-value pairs into it
// and then performs some operations on the map,
// including printing all the keys and values, checking if a specific key is present,
// and removing a key-value pair. 
use std::collections::HashMap;
fn main() {
    let mut map = HashMap::new();
    map.insert("John Coltrane", "A Love Supreme");
    map.insert("Miles Davis", "In a Silent Way");
    map.insert("Pharoah Sanders", "The Creator has a Masterplan");

    // prints all keys and values in the map
    for (key, value) in &map {
        println!("{}: {}:", key, value);
    }

    // check if a key is present in the map
    let key = "key2";
    if map.contains_key(key) {
        println!("The map contains the key '{}'", key);
    } else {
        println!("The map do NOT contains the key '{}'", key);
        
    }
    
    // remove a key-value pair from the map
    let removed = map.remove("Miles Davis");
    match removed {
        
        Some(value) => println!("Removed key-value pair 'Miles Davis' '{}'", value),
        None => println!("Key not found in the map")
    }

    // print the map again
    for (key, value) in &map {
        println!("{}: {}", key, value);
        
    }

}