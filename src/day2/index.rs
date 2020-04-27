use crate::utils;
use std::io::{BufRead, Result};

pub fn main() -> Result<u64> {
    let final_value = restore_state()?;
    Ok(final_value)
}

fn restore_state() -> Result<u64> {
    let mut reader = utils::index::read_file("input_day2.txt")?;
    let mut raw_data = String::new();
    reader
        .read_line(&mut raw_data)
        .ok()
        .expect("Unable to read line in the input_day2.txt");

    let mut numbers: Vec<u64> = raw_data
        .split(",")
        .map(|s| s.parse().expect("Unable to parse the number"))
        .collect();
    let mut index = 0;

    while index <= numbers.len() {
        let (new_value, must_halt) = process_opcode(&numbers, &index);
        if must_halt == true {
            break;
        };
        let idx_to_update = numbers[index + 3];
        numbers[idx_to_update as usize] = new_value;
        index += 4;
    }

    Ok(numbers[0])
}

fn process_opcode(numbers: &Vec<u64>, index: &usize) -> (u64, bool) {
    let opcode = numbers[*index];
    let num_1_pos = numbers[*index + 1];
    let num_2_pos = numbers[*index + 2];

    if opcode == 99 {
        return (0, true);
    } else if opcode == 1 {
        return (
            numbers[num_1_pos as usize] + numbers[num_2_pos as usize],
            false,
        );
    } else {
        return (
            numbers[num_1_pos as usize] * numbers[num_2_pos as usize],
            false,
        );
    }
}
