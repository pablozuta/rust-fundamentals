struct Weapon {
    name: String,
    damage: u32,
    fire_rate: u32,
}

impl Weapon {
    fn new(name: String, damage: u32, fire_rate: u32) -> Self {
        Weapon{
            name, 
            damage,
            fire_rate,
        }
    }
    fn calculate_dps(&self) -> u32 {
        self.damage * self.fire_rate
    }
    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Damage: {}", self.damage);
        println!("Fire Rate: {}", self.fire_rate);
        println!("DPS:{}", self.calculate_dps());
        println!();
    }
}

fn main() {
    // sample data from cyberpunk 2077
    let weapons = [
        Weapon::new(String::from("Malorian Arms 3516"), 150, 5),
        Weapon::new(String::from("Kang Tao Dian"), 120, 8),
        Weapon::new(String::from("Arasaka Mantis Blade"), 100, 10),
    ];

    // display info for each weapon
    for weapon in &weapons {
        weapon.display_info();
    }
}