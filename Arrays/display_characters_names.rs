fn main() {
    let characters_names = ["V", "Johnny Silverhand", "Takemura", "Panam Palmer", "Judy Alvarez"];

    // display all characters names
    let mut count = 1;
    for name in &characters_names {
        println!("Character {}: {}", count, name);
        count += 1;
    }
}