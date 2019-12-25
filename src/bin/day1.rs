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
        print!("Mass: {}, Fuel: {}", module_mass, module_fuel);

        total_fuel += module_fuel;

        let mut addtl_fuel = (module_fuel as i32 / 3) - 2;
        while addtl_fuel > 0 {
            print!(" + {}", addtl_fuel);
            total_fuel += addtl_fuel as u32;
            addtl_fuel = (addtl_fuel / 3) - 2;
        }
        println!();
    }

    println!("Total fuel: {}", total_fuel);

    Ok(())
}
