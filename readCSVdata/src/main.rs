extern crate csv;

use std::error::Error;
use std::fs::File;
 use std::process;

fn main() -> Result<(), Box<dyn Error>> {
    // open the csv file
    let file = File::open("cyberpunk.csv")?;
    let mut rdr = csv::Reader::from_reader(file);

    // read the headers
    let headers = rdr.Headers()?.clone();

    // read the data into a vector of tuples
    let data: Vec<(String, i32, i32, i32)> = rdr.records().map(|r| {
        let record = r?;
        let name = record.get(0).unwrap().to_string();
        let level = record.get(1).unwrap().parse::<i32>().unwrap();
        let damage = record.get(2).unwrap().parse::<i32>().unwrap();
        let armor = record.get(3).unwrap().parse::<i32>().unwrap();
        Ok((name, level, damage, armor))
    })
    .collect::<Result<_, csv::Error>>()?;

    // print the data
    for (name, level, damage, armor) in &data {
        println!("Name: {}, Level: {}, Damage: {}, Armor: {}", name, level, damage, armor);
    }

    Ok(())
}