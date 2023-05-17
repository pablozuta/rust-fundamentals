struct Character {
    name: String,
    level: u8,
    role: String,
    street_cred: u32,
}

impl Character {
    fn new(name: String, level: u8, role: String, street_cred: u32 ) -> Self {
        Character{
            name,
            level,
            role,
            street_cred,
        }
    }
    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Level: {}", self.level);
        println!("Role: {}", self.role);
        println!("Street Cred:{}", self.street_cred);
        println!();
    }
}

fn main() {
    // sample character info from cyberpunk 2077
    let characters = [
        Character::new(String::from("V"), 50, String::from("Mercenary"), 1500),
        Character::new(String::from("Johnny Silverhand"), 45, String::from("Rockerboy"), 1200),
        Character::new(String::from("Panam Palmer"), 40, String::from("Nomad"), 1000),
    ];

    // display info for each character
    for character in &characters {
        character.display_info();
    }
}