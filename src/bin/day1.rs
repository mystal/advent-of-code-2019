// https://adventofcode.com/2019/day/1

use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("input/day1.txt")?;
    let reader = BufReader::new(f);

    let mut total_fuel = 0;

    for line in reader.lines() {
        let module_mass: u32 = line?.parse()?;
        let module_fuel = (module_mass / 3) - 2;
        println!("Mass: {}, Fuel: {}", module_mass, module_fuel);

        total_fuel += module_fuel;
    }

    println!("Total fuel: {}", total_fuel);

    Ok(())
}
