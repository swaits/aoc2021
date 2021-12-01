use crate::utils;
use anyhow::Result;

fn count_incrementing_windows(window: usize, v: &[i64]) -> usize {
    v[window..].iter().zip(v).filter(|(b, a)| b > a).count()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let data = utils::read_i64s("data/day01.txt")?;
    Ok((
        count_incrementing_windows(1, &data),
        count_incrementing_windows(3, &data),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let data = utils::read_i64s("data/day01-test.txt").unwrap();
        assert_eq!(count_incrementing_windows(1, &data), 7);
        assert_eq!(count_incrementing_windows(3, &data), 5);
    }

    #[test]
    fn test_my_data() {
        let data = utils::read_i64s("data/day01.txt").unwrap();
        assert_eq!(count_incrementing_windows(1, &data), 1298);
        assert_eq!(count_incrementing_windows(3, &data), 1248);
    }
}
