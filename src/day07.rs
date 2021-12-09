use anyhow::Result;

use crate::utils::parse_by_char;

fn get_median(positions: &mut [isize]) -> isize {
    let median_index = positions.len() / 2;
    positions.select_nth_unstable(median_index); // quickselect in the standard library!
    positions[median_index]
}

fn compute_fuel(positions: &[isize], target: isize, cheap_fuel: bool) -> usize {
    positions
        .iter()
        .map(|p| {
            let length = (p - target).abs() as usize;
            if cheap_fuel {
                length
            } else {
                length * (length + 1) / 2
            }
        })
        .sum()
}

fn find_cheapest_location(positions: &mut [isize], cheap_fuel: bool) -> usize {
    // find start point
    let start = get_median(positions);
    let mut position = start;
    let mut fuel = compute_fuel(positions, position, cheap_fuel);

    // search up
    loop {
        let new_fuel = compute_fuel(positions, position + 1, cheap_fuel);
        if new_fuel > fuel {
            break;
        }
        position += 1;
        fuel = new_fuel;
    }
    if position != start {
        return fuel;
    }

    // search down
    loop {
        let new_fuel = compute_fuel(positions, position - 1, cheap_fuel);
        if new_fuel > fuel {
            break;
        }
        position -= 1;
        fuel = new_fuel;
    }

    // done
    fuel
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let mut data = parse_by_char::<isize>(include_str!("../data/day07.txt"), ',').unwrap();
    Ok((
        find_cheapest_location(&mut data, true),
        find_cheapest_location(&mut data, false),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let mut data = parse_by_char::<isize>(include_str!("../data/day07-test.txt"), ',').unwrap();
        assert_eq!(find_cheapest_location(&mut data, true), 37);
        assert_eq!(find_cheapest_location(&mut data, false), 168);
    }

    #[test]
    fn test_my_data() {
        let mut data = parse_by_char::<isize>(include_str!("../data/day07.txt"), ',').unwrap();
        assert_eq!(find_cheapest_location(&mut data, true), 344138);
        assert_eq!(find_cheapest_location(&mut data, false), 94862124);
    }
}
