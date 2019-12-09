use math::round;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::path::Path;

fn get_sum_fuel_requirements<P>(filename: P) -> Result<f64>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    let reader = BufReader::new(file);
    let mut sum: f64 = 0.0;

    for line in reader.lines() {
        if let Ok(module_mass_str) = line {
            let mass: f64 = module_mass_str.trim().parse().expect("Not a valid integer");
            let requirement = round::floor(mass / 3.0, 0) - 2.0;
            sum += requirement;
        }
    }

    Ok(sum)
}

fn main() -> Result<()> {
    let requirement = get_sum_fuel_requirements("input.txt")?;
    println!("{}", requirement);
    Ok(())
}
