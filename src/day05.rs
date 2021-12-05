use anyhow::Result;
use itertools::Itertools;

#[derive(Debug)]
struct LineSegment {
    start: (isize, isize),
    end: (isize, isize),
}

impl LineSegment {
    fn is_horizontal_or_vertical(&self) -> bool {
        self.start.0 == self.end.0 || self.start.1 == self.end.1
    }

    fn rasterize(&self) -> Vec<(isize, isize)> {
        let xstep = (self.end.0 - self.start.0).signum();
        let ystep = (self.end.1 - self.start.1).signum();
        let mut ret = vec![self.start];
        let (mut x, mut y) = self.start;
        while (x, y) != self.end {
            x += xstep;
            y += ystep;
            ret.push((x, y));
        }
        ret
    }
}

fn parse_lines(input: &str) -> Vec<LineSegment> {
    input
        .lines()
        .map(|l| {
            let mut parts = l.split_whitespace();
            let start_str = parts.next().unwrap();
            let end_str = parts.nth(1).unwrap();
            LineSegment {
                start: start_str
                    .split(',')
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
                end: end_str
                    .split(',')
                    .map(|s| s.parse::<isize>().unwrap())
                    .collect_tuple()
                    .unwrap(),
            }
        })
        .collect()
}

fn count_overlapped_cells(segments: &[LineSegment], includ_diag: bool) -> usize {
    let mut marks: Vec<(isize, isize)> = segments
        .iter()
        .filter(|s| s.is_horizontal_or_vertical() || includ_diag)
        .map(|s| s.rasterize())
        .flatten()
        .collect();
    // sort so we can find duplicates
    // **NOTE** IterTools has `.duplicates()` but it's significantly slower (more than 2x) than this
    marks.sort_unstable();
    marks[..]
        .iter()
        .zip(&marks[1..])
        .filter(|(a, b)| a == b)
        .unique()
        .count()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = include_str!("../data/day05.txt");
    let segments = parse_lines(input);
    Ok((
        count_overlapped_cells(&segments, false),
        count_overlapped_cells(&segments, true),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = include_str!("../data/day05-test.txt");
        let segments = parse_lines(input);
        assert_eq!(count_overlapped_cells(&segments, false), 5);
        assert_eq!(count_overlapped_cells(&segments, true), 12);
    }

    #[test]
    fn test_my_data() {
        let input = include_str!("../data/day05.txt");
        let segments = parse_lines(input);
        assert_eq!(count_overlapped_cells(&segments, false), 5124);
        assert_eq!(count_overlapped_cells(&segments, true), 19771);
    }
}
