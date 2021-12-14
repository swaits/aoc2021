use std::env;

use anyhow::Result;

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod utils;

// used to generate more runs when profiling
const RUNS: usize = 1;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    for _ in 0..RUNS {
        for (i, f) in [
            day01::run,
            day02::run,
            day03::run,
            day04::run,
            day05::run,
            day06::run,
            day07::run,
            day08::run,
            day09::run,
            day10::run,
        ]
        .iter()
        .enumerate()
        {
            if args.len() == 1 || args.contains(&(i + 1).to_string()) {
                let result = f()?;
                if RUNS == 1 {
                    println!("Day {:02} -> {:?}", i + 1, result);
                }
            }
        }
    }

    Ok(())
}
