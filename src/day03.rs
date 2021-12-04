use crate::utils::read_file;
use anyhow::Result;

fn strings_to_ints(s: &str) -> Vec<usize> {
    s.lines()
        .map(|line| usize::from_str_radix(line, 2).unwrap())
        .collect()
}

fn diagnose_power(s: &str) -> usize {
    // collect all positions with ones
    let bits = s.lines().next().unwrap().len();
    let n_words = s.lines().count();
    let mut counts = vec![0; bits];
    s.lines().for_each(|line| {
        line.chars().enumerate().for_each(|(i, c)| {
            if c == '1' {
                counts[i] += 1;
            }
        })
    });

    // determine whether zero or one is greatter than half for each position
    let ret: String = counts
        .iter()
        .map(|n| if n > &(n_words / 2) { '1' } else { '0' })
        .collect();

    // put together gamma (all majority 1 slots) and epsilon (!gamma)
    let gamma = usize::from_str_radix(&ret, 2).unwrap();
    let epsilon = !gamma & ((1 << bits) - 1);
    gamma * epsilon
}

fn find_oxygen_or_co2(s: &str, oxygen: bool) -> usize {
    let bits = s.lines().next().unwrap().len();
    let mut kept = strings_to_ints(s);
    usize::from_str_radix(
        &(0..bits)
            .rev()
            .map(|bit| {
                // split current set into ones/zeroes for current bit
                let mut ones = vec![];
                let mut zeroes = vec![];
                kept.iter().for_each(|x| {
                    if x & (1 << bit) != 0 {
                        ones.push(*x);
                    } else {
                        zeroes.push(*x);
                    }
                });

                if kept.len() == 1 {
                    // only one left so we take its 0 or 1 regardless
                    if kept[0] & (1 << bit) != 0 {
                        '1'
                    } else {
                        '0'
                    }
                } else if oxygen {
                    // keep whichever has the most, ties are 1
                    if ones.len() >= zeroes.len() {
                        kept = ones;
                        '1'
                    } else {
                        kept = zeroes;
                        '0'
                    }
                } else {
                    // keep whichever has the least, ties are 0
                    if zeroes.len() <= ones.len() {
                        kept = zeroes;
                        '0'
                    } else {
                        kept = ones;
                        '1'
                    }
                }
            })
            .collect::<String>(),
        2,
    )
    .unwrap()
}
fn diagnose_lifesupport(s: &str) -> usize {
    let oxygen_generator = find_oxygen_or_co2(s, true);
    let co2_scrubber = find_oxygen_or_co2(s, false);
    oxygen_generator * co2_scrubber
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = read_file("data/day03.txt")?;
    Ok((diagnose_power(&input), diagnose_lifesupport(&input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_file("data/day03-test.txt").unwrap();
        assert_eq!(diagnose_power(&input), 198);
        assert_eq!(diagnose_lifesupport(&input), 230);
    }

    #[test]
    fn test_my_data() {
        let input = read_file("data/day03.txt").unwrap();
        assert_eq!(diagnose_power(&input), 4191876);
        // assert_eq!(follow_course(&input), 2070300);
        // assert_eq!(follow_course_better(&input), 2078985210);
    }
}
