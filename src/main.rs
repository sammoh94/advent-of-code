use std::io::Result;

mod day1;
mod day2;
mod utils;

fn main() -> Result<()> {
    let answer = day2::index::main()?;
    println!("{}", answer);
    Ok(())
}
