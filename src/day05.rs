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

    fn rasterize(&self) -> LineSegmentPixelIter {
        let dx = self.end.0 - self.start.0;
        let dy = self.end.1 - self.start.1;
        LineSegmentPixelIter {
            x: self.start.0,
            y: self.start.1,
            xstep: dx.signum(),
            ystep: dy.signum(),
            length: dx.abs().max(dy.abs()) + 1,
            i: 0,
        }
    }
}

struct LineSegmentPixelIter {
    x: isize,
    y: isize,
    xstep: isize,
    ystep: isize,
    length: isize,
    i: isize,
}

impl Iterator for LineSegmentPixelIter {
    type Item = (isize, isize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.i < self.length {
            let ret = (self.x + self.i * self.xstep, self.y + self.i * self.ystep);
            self.i += 1;
            Some(ret)
        } else {
            None
        }
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
    let mut cells: Vec<Vec<usize>> = vec![vec![0; 1000]; 1000];
    segments
        .iter()
        .filter(|s| s.is_horizontal_or_vertical() || includ_diag)
        .map(|s| {
            s.rasterize()
                .map(|(x, y)| -> usize {
                    cells[x as usize][y as usize] += 1;
                    if cells[x as usize][y as usize] == 2 {
                        1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>()
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
