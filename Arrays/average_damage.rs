fn main() {
    let weapon_damages = [45, 62, 78, 55, 40];

    // calculate the average weapon damage
    let total_damage: i32 = weapon_damages.iter().sum();
    let average_damage = (total_damage as f32) / (weapon_damages.len() as f32);

    // display the result
    println!("Average weapon damage: {:.2}", average_damage);
}