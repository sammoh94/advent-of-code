use math::round;
use std::io::{BufRead, Result};

use crate::utils;

pub fn main() -> Result<f64> {
    let requirement = get_sum_fuel_requirements()?;
    Ok(requirement)
}

fn get_sum_fuel_requirements() -> Result<f64> {
    let reader = utils::index::read_file("input_day1.txt")?;
    let mut sum: f64 = 0.0;

    for line in reader.lines() {
        if let Ok(module_mass_str) = line {
            let mass: f64 = module_mass_str.trim().parse().expect("Not a valid integer");
            sum += compute_module_fuel_needs(mass);
        }
    }

    Ok(sum)
}

fn compute_module_fuel_needs(fuel: f64) -> f64 {
    let mut sum = 0.0;
    let mut requirement = compute_requirement(fuel);

    while !(requirement < 0.0) {
        sum += requirement;
        requirement = compute_requirement(requirement);
    }

    sum
}

fn compute_requirement(fuel: f64) -> f64 {
    round::floor(fuel / 3.0, 0) - 2.0
}
