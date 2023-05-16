use std::collections::HashSet;

fn main() {
    let mut set = HashSet::new();
    set.insert("banana");
    set.insert("apple");
    set.insert("peach");

    // Print all elements on the set
    for element in &set {
        println!("{}", element);
    }

    // Check if element is present on the set
    let element = "banana";
    if set.contains(element) {
        println!("Element '{}' belong to the set.", element);
    } else {
        println!("Element {} DON'T belong to the set.", element);
    }

    // Remove an element from the set
    let removed = set.take(&"peach");
    match removed {
        Some(element) => println!("Removed element: {}", element),
        None => println!("Elementnot found on the set"),
    }
}