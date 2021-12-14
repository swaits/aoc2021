use anyhow::Result;
use itertools::Itertools;

// return the number of chararcters present in both strings
// NOTE: this is O(n^2), but our input is tiny
//       a HashSet is better complexity, but the allocations will hurt us
fn shared_chars(a: &str, b: &str) -> usize {
    if a.len() < b.len() {
        a.chars().filter(|ch| b.contains(&ch.to_string())).count()
    } else {
        b.chars().filter(|ch| a.contains(&ch.to_string())).count()
    }
}

// decode a set of 10 input + 4 output characters and return the output
fn descramble(words: &[&str]) -> usize {
    // println!("input = {:?}, output = {:?}", input, output);
    let mut s1_index: Option<usize> = None;
    let mut s4_index: Option<usize> = None;

    // find the 1 and 4
    words.iter().enumerate().for_each(|(i, w)| match w.len() {
        2 => s1_index = Some(i),
        4 => s4_index = Some(i),
        _ => (),
    });
    assert!(s1_index.is_some() && s4_index.is_some());

    // now decode each digit based on delta from the 1 and 4 digits
    let result = words
        .iter()
        .map(|w| match w.len() {
            2 => '1',
            3 => '7',
            4 => '4',
            5 => {
                // 2, 3, or 5
                if shared_chars(words[s1_index.unwrap()], w) == 2 {
                    '3'
                } else if shared_chars(words[s4_index.unwrap()], w) == 3 {
                    '5'
                } else {
                    '2'
                }
            }
            6 => {
                // 0, 6, or 9
                if shared_chars(words[s1_index.unwrap()], w) == 1 {
                    '6'
                } else if shared_chars(words[s4_index.unwrap()], w) == 4 {
                    '9'
                } else {
                    '0'
                }
            }
            7 => '8',
            _ => unreachable!(),
        })
        .collect::<String>();

    // return the last for digits as a usize
    (&result[10..]).parse::<usize>().unwrap()
}

fn part1(s: &str) -> usize {
    s.split_whitespace()
        .chunks(15)
        .into_iter()
        .map(|chunk| {
            chunk
                .skip(11)
                .filter(|token| [2, 4, 3, 7].contains(&token.len()))
                .count()
        })
        .sum()
}

fn part2(s: &str) -> usize {
    s.split_whitespace()
        .chunks(15)
        .into_iter()
        .map(|chunk| {
            let mut words: Vec<&str> = chunk.collect();
            words.remove(10);
            descramble(&words)
        })
        .sum()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = include_str!("../data/day08.txt");
    Ok((part1(input), part2(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../data/day08-test.txt");
        assert_eq!(part1(input), 26);
        assert_eq!(part2(input), 61229);
    }

    #[test]
    fn test_my_data() {
        let input = include_str!("../data/day08.txt");
        assert_eq!(part1(input), 272);
        assert_eq!(part2(input), 1007675);
    }
}
