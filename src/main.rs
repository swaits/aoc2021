use anyhow::Result;

mod day01;
mod utils;

fn main() -> Result<()> {
    println!("day01: {:?}", day01::run()?);
    Ok(())
}
