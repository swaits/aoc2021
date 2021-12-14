use anyhow::Result;
use itertools::Itertools;

fn matching(c: char) -> char {
    match c {
        ')' => '(',
        ']' => '[',
        '}' => '{',
        '>' => '<',
        '(' => ')',
        '[' => ']',
        '{' => '}',
        '<' => '>',
        _ => unreachable!(),
    }
}

enum Response {
    Illegal(char),
    Incomplete(String),
    Ok,
}

fn check_line(s: &str) -> Response {
    let mut stack = Vec::new();
    for c in s.trim().chars() {
        if ['(', '[', '{', '<'].contains(&c) {
            stack.push(c)
        } else if matching(c) != stack.pop().unwrap() {
            return Response::Illegal(c);
        }
    }

    if !stack.is_empty() {
        Response::Incomplete(stack.iter().rev().map(|c| matching(*c)).collect())
    } else {
        Response::Ok
    }
}

fn sum_illegal_chars(s: &str) -> usize {
    s.lines()
        .map(|l| match check_line(l) {
            Response::Illegal(')') => 3,
            Response::Illegal(']') => 57,
            Response::Illegal('}') => 1197,
            Response::Illegal('>') => 25137,
            Response::Incomplete(_) => 0,
            Response::Ok => 0,
            _ => unreachable!(),
        })
        .sum()
}

fn score_incomplete_lines(s: &str) -> usize {
    let mut scores = s
        .lines()
        .filter_map(|l| match check_line(l) {
            Response::Illegal(_) => None,
            Response::Incomplete(s) => {
                let mut score = 0;
                s.chars().for_each(|c| {
                    score *= 5;
                    score += match c {
                        ')' => 1,
                        ']' => 2,
                        '}' => 3,
                        '>' => 4,
                        _ => unreachable!(),
                    }
                });
                Some(score)
            }
            Response::Ok => None,
        })
        .collect_vec();

    // select the N/2'th element
    let median_index = scores.len() / 2;
    scores.select_nth_unstable(median_index);
    scores[median_index]
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = include_str!("../data/day10.txt");
    Ok((sum_illegal_chars(input), score_incomplete_lines(input)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../data/day10-test.txt");
        assert_eq!(sum_illegal_chars(input), 26397);
        assert_eq!(score_incomplete_lines(input), 288957);
    }

    #[test]
    fn test_my_data() {
        let input = include_str!("../data/day10.txt");
        assert_eq!(sum_illegal_chars(input), 390993);
        assert_eq!(score_incomplete_lines(input), 2391385187);
    }
}
