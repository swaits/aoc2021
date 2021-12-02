use crate::utils::read_file;
use anyhow::Result;

fn parse_line(s: &str) -> (&str, i32) {
    let parsed: Vec<&str> = s.splitn(2, ' ').collect();
    let instruction = parsed[0];
    let quantity = parsed[1].parse::<i32>().unwrap();
    (instruction, quantity)
}

fn follow_course(s: &str) -> i32 {
    let (mut pos, mut depth) = (0, 0);
    s.lines().for_each(|line| match parse_line(line) {
        ("forward", d) => pos += d,
        ("down", d) => depth += d,
        ("up", d) => depth -= d,
        _ => unreachable!(),
    });
    pos * depth
}

fn follow_course_better(s: &str) -> i32 {
    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    s.lines().for_each(|line| match parse_line(line) {
        ("forward", d) => {
            pos += d;
            depth += aim * d;
        }
        ("down", d) => aim += d,
        ("up", d) => aim -= d,
        _ => unreachable!(),
    });
    pos * depth
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = read_file("data/day02.txt")?;
    Ok((
        follow_course(&input) as usize,
        follow_course_better(&input) as usize,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_file("data/day02-test.txt").unwrap();
        assert_eq!(follow_course(&input), 150);
        assert_eq!(follow_course_better(&input), 900);
    }

    #[test]
    fn test_my_data() {
        let input = read_file("data/day02.txt").unwrap();
        assert_eq!(follow_course(&input), 2070300);
        assert_eq!(follow_course_better(&input), 2078985210);
    }
}
