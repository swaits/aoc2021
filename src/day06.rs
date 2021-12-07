use anyhow::Result;
use cached::proc_macro::cached;

use crate::utils::parse_by_char;

#[cached]
fn count_babies(birthday: isize, end_day: isize) -> usize {
    1 + (birthday..=end_day)
        .skip(2 + 7)
        .step_by(7)
        .map(|day| count_babies(day, end_day) as usize)
        .sum::<usize>()
}

fn count_population(pop: &Vec<isize>, end_day: isize) -> usize {
    pop.iter().map(|d| count_babies(d - 8, end_day)).sum()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let data = parse_by_char::<isize>(include_str!("../data/day06.txt"), ',')?;
    Ok((count_population(&data, 80), count_population(&data, 256)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn emit_babies() {
        assert_eq!(count_babies(3 - 8, 18), 5);
    }

    #[test]
    fn test_example() {
        let data = parse_by_char::<isize>(include_str!("../data/day06-test.txt"), ',').unwrap();
        assert_eq!(count_population(&data, 18), 26);
        assert_eq!(count_population(&data, 80), 5934);
        assert_eq!(count_population(&data, 256), 26984457539);
    }

    #[test]
    fn test_my_data() {
        let data = parse_by_char::<isize>(include_str!("../data/day06.txt"), ',').unwrap();
        assert_eq!(count_population(&data, 80), 358214);
    }
}
