struct Weapon {
    name: String,
    damage: u32,
    fire_rate: u32,
}

impl Weapon {
    fn new(name:String, damage: u32, fire_rate: u32) -> Self {
        Weapon{
            name,
            damage,
            fire_rate,
        }
    }
    fn calculate_power(&self) -> u32 {
        self.damage * self.fire_rate
    }

    fn display_info(&self) {
        println!("Name: {}", self.name);
        println!("Damage: {}", self.damage);
        println!("Fire Rate: {}", self.fire_rate);
        println!();
    }
}
fn main() {
    // sample data from cyberpunk 2077
    let weapons = [
        Weapon::new(String::from("Malorian Arms 3516"), 150, 5),
        Weapon::new(String::from("Kang Tao Dian"), 120, 8),
        Weapon::new(String::from("Arasaka Mantis Blades"), 100, 10),
    ];

    // find the most powerful weapon
    let mut most_powerful_weapon = &weapons[0];
    for weapon in &weapons {
        if weapon.calculate_power() > most_powerful_weapon.calculate_power() {
            most_powerful_weapon = weapon;
        }
    }
    // display info of the most powerful weapon
    println!("***Most Powerful Weapon***");
    most_powerful_weapon.display_info();

}