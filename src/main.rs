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
    let dayfns = [
        day01::run,
        day02::run,
        day03::run,
        day04::run,
        day05::run,
        day06::run,
        day07::run,
    ];

    for (i, f) in dayfns.iter().enumerate() {
        println!("Day {:02} -> {:?}", i, f()?);
    }

    Ok(())
}
