use anyhow::Result;

mod day01;
mod day02;
mod utils;

fn main() -> Result<()> {
    println!("day01: {:?}", day01::run()?);
    println!("day02: {:?}", day02::run()?);
    Ok(())
}
