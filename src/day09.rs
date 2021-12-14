use std::collections::HashSet;

use anyhow::{anyhow, Result};
use itertools::Itertools;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Coordinate {
    x: usize,
    y: usize,
}

struct HeightMap {
    data: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl HeightMap {
    fn parse(s: &str) -> Result<Self> {
        let data = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| match c {
                        '0' => 0,
                        '1' => 1,
                        '2' => 2,
                        '3' => 3,
                        '4' => 4,
                        '5' => 5,
                        '6' => 6,
                        '7' => 7,
                        '8' => 8,
                        '9' => 9,
                        _ => unreachable!(),
                    })
                    .collect_vec()
            })
            .collect_vec();
        let width = data[0].len();
        let height = data.len();
        if !data.is_empty() {
            Ok(HeightMap {
                data,
                width,
                height,
            })
        } else {
            Err(anyhow!("HeightMap parse error"))
        }
    }

    fn neighbors(&self, pos: &Coordinate) -> Vec<Coordinate> {
        let mut ret = vec![];
        if pos.x > 0 {
            ret.push(to_coord(pos.x - 1, pos.y))
        }
        if pos.y > 0 {
            ret.push(to_coord(pos.x, pos.y - 1))
        }
        if pos.x < self.width - 1 {
            ret.push(to_coord(pos.x + 1, pos.y))
        }
        if pos.y < self.height - 1 {
            ret.push(to_coord(pos.x, pos.y + 1))
        }
        ret
    }
    fn iter(&self) -> impl Iterator<Item = Coordinate> {
        (0..self.height)
            .cartesian_product(0..self.width)
            .map(|(y, x)| Coordinate { x, y })
    }

    fn value_at(&self, pos: &Coordinate) -> Option<usize> {
        if pos.x < self.width && pos.y < self.height {
            Some(self.data[pos.y][pos.x])
        } else {
            None
        }
    }

    fn is_local_minima(&self, pos: &Coordinate) -> bool {
        let value = self.value_at(pos).unwrap();
        self.neighbors(pos)
            .iter()
            .all(|n| self.value_at(n).unwrap() > value)
    }

    fn find_basin_members(&self, root: &Coordinate) -> HashSet<Coordinate> {
        // `root` is assumed to be in the basin
        let mut ret: HashSet<Coordinate> = HashSet::new();
        ret.insert(*root);
        let value = self.value_at(root).unwrap();

        // test each of the qualifying neighbors of `root`
        let good_neighbors = self
            .neighbors(root)
            .iter()
            .filter(|npos| {
                let nvalue = self.value_at(npos).unwrap();
                nvalue > value && nvalue < 9
            })
            .cloned()
            .collect_vec();

        // recursively search for each of the qualifying neighbors' neighbors
        good_neighbors
            .iter()
            .for_each(|n| ret.extend(self.find_basin_members(n).iter()));

        // glue it all together and return
        ret
    }
}

fn to_coord(x: usize, y: usize) -> Coordinate {
    Coordinate { x, y }
}

fn find_low_points(heightmap: &HeightMap) -> Vec<Coordinate> {
    heightmap
        .iter()
        .filter(|coord| heightmap.is_local_minima(coord))
        .collect_vec()
}

// scan the whole grid for low points
fn sum_low_points(heightmap: &HeightMap) -> usize {
    find_low_points(heightmap)
        .iter()
        .map(|coord| heightmap.value_at(coord).unwrap() + 1)
        .sum()
}

fn sum_top3_basins(heightmap: &mut HeightMap) -> usize {
    let mut basins = find_low_points(heightmap)
        .iter()
        .map(|p| heightmap.find_basin_members(p).len())
        .collect_vec();
    basins.sort_unstable();
    basins.iter().rev().take(3).product()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let mut heightmap = HeightMap::parse(include_str!("../data/day09.txt"))?;
    Ok((sum_low_points(&heightmap), sum_top3_basins(&mut heightmap)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut heightmap = HeightMap::parse(include_str!("../data/day09-test.txt")).unwrap();
        assert_eq!(sum_low_points(&heightmap), 15);
        assert_eq!(sum_top3_basins(&mut heightmap), 1134);
    }

    #[test]
    fn test_my_data() {
        let heightmap = HeightMap::parse(include_str!("../data/day09.txt")).unwrap();
        assert_eq!(sum_low_points(&heightmap), 504);
    }
}
