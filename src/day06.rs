use anyhow::Result;

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = include_str!("../data/day06.txt");
    Ok((0, 0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../data/day06-test.txt");
    }

    #[test]
    fn test_my_data() {
        let input = include_str!("../data/day06.txt");
    }
}
