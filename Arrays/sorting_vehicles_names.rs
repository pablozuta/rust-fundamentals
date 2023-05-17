fn main() {
    // sample vehicles names from cyberpunk 2077
    let mut vehicle_names = ["Quadra Type-66", "Thorton Colby", "Rayfield Caliburn", "Yaiba Kusanagi"];

    // sort by alphabetical order
    vehicle_names.sort();

    // display the sorted list
    for name in &vehicle_names {
        println!("{}", name);
    }
}