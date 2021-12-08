use anyhow::Result;

fn parse_line(s: &str) -> (char, i32) {
    let mut parsed = s.split_ascii_whitespace();
    let instruction = match parsed.next().unwrap() {
        "forward" => 'f',
        "down" => 'd',
        "up" => 'u',
        _ => unreachable!(),
    };
    let quantity = parsed.next().unwrap().parse::<i32>().unwrap();
    (instruction, quantity)
}

fn follow_course(s: &str) -> i32 {
    let (mut pos, mut depth) = (0, 0);
    s.lines().for_each(|line| match parse_line(line) {
        ('f', d) => pos += d,
        ('d', d) => depth += d,
        ('u', d) => depth -= d,
        _ => unreachable!(),
    });
    pos * depth
}

fn follow_course_better(s: &str) -> i32 {
    let (mut pos, mut depth, mut aim) = (0, 0, 0);
    s.lines().for_each(|line| match parse_line(line) {
        ('f', d) => {
            pos += d;
            depth += aim * d;
        }
        ('d', d) => aim += d,
        ('u', d) => aim -= d,
        _ => unreachable!(),
    });
    pos * depth
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = include_str!("../data/day02.txt");
    Ok((
        follow_course(input) as usize,
        follow_course_better(input) as usize,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../data/day02-test.txt");
        assert_eq!(follow_course(input), 150);
        assert_eq!(follow_course_better(input), 900);
    }

    #[test]
    fn test_my_data() {
        let input = include_str!("../data/day02.txt");
        assert_eq!(follow_course(input), 2070300);
        assert_eq!(follow_course_better(input), 2078985210);
    }
}
