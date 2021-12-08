use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod utils;

fn main() -> Result<()> {
    println!("day01: {:?}", day01::run()?);
    println!("day02: {:?}", day02::run()?);
    println!("day03: {:?}", day03::run()?);
    println!("day04: {:?}", day04::run()?);
    println!("day05: {:?}", day05::run()?);
    println!("day06: {:?}", day06::run()?);
    println!("day07: {:?}", day07::run()?);
    Ok(())
}
