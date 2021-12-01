use anyhow::Result;
use std::fs;

fn read_i64s(filename: &str) -> Result<Vec<i64>> {
    Ok(read_file(filename)?
        .lines()
        .map(|s| s.parse::<i64>().expect("i64 parse error"))
        .collect())
}

fn read_file(filename: &str) -> Result<String> {
    Ok(fs::read_to_string(filename)?)
}

fn count_incrementing_windows(window: usize, v: &[i64]) -> usize {
    // create an array of the window sums and then window(2) that array to count increments
    v.windows(window)
        .map(|w| w.iter().sum())
        .collect::<Vec<i64>>()
        .windows(2)
        .filter(|s| s[1] > s[0])
        .count()
}

fn day01() -> Result<(usize, usize)> {
    let data = read_i64s("data/day01-test.txt")?;
    assert_eq!(count_incrementing_windows(1, &data), 7);
    assert_eq!(count_incrementing_windows(3, &data), 5);

    let data = read_i64s("data/day01.txt")?;
    let part1 = count_incrementing_windows(1, &data);
    let part2 = count_incrementing_windows(3, &data);
    assert_eq!(part1, 1298);
    assert_eq!(part2, 1248);
    Ok((part1, part2))
}

fn main() -> Result<()> {
    println!("day01: {:?}", day01()?);
    Ok(())
}
