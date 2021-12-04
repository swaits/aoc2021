use crate::utils::read_file;
use anyhow::Result;

#[derive(Debug)]
struct Bingo {
    draws: Vec<usize>,
    boards: Vec<Vec<usize>>,
}

fn parse_bingo(s: &str) -> Bingo {
    Bingo {
        // take the first line and split on comma
        draws: s
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<usize>().unwrap())
            .collect(),
        // split on whitespace, collect all the remaining numbers and then chunk them out in 25s
        boards: s
            .lines()
            .skip(2)
            .map(|line| line.split_whitespace().map(|s| s.parse::<usize>().unwrap()))
            .flatten()
            .collect::<Vec<usize>>()
            .chunks(25)
            .map(|c| c.into())
            .collect(),
    }
}

fn test_for_win(marked: usize) -> bool {
    let col_mask: usize = 0b1000010000100001000010000;
    let row_mask: usize = 0b1111100000000000000000000;
    (0..5).any(|i| {
        marked & (col_mask >> i) == (col_mask >> i)
            || marked & (row_mask >> (i * 5)) == (row_mask >> (i * 5))
    })
}

fn compute_score(board: &[usize], marked: usize, last_draw: usize) -> usize {
    (0..25)
        .map(|i| if marked & 1 << i == 0 { board[i] } else { 0 })
        .sum::<usize>()
        * last_draw
}

fn find_winning_score(s: &str, first: bool) -> usize {
    let bingo = parse_bingo(s);
    let mut markers = vec![0; bingo.boards.len()];
    let mut boards_won = vec![false; bingo.boards.len()];
    // for every drawn number, mark each board and check whether it wins
    for draw in bingo.draws {
        for i in 0..bingo.boards.len() {
            for j in 0..bingo.boards[i].len() {
                if bingo.boards[i][j] == draw {
                    markers[i] |= 1 << j;
                }
                if test_for_win(markers[i]) {
                    boards_won[i] = true;
                    if first || (!first && boards_won.iter().all(|w| *w)) {
                        return compute_score(&bingo.boards[i], markers[i], draw);
                    }
                }
            }
        }
    }
    unreachable!()
}

pub(crate) fn run() -> Result<(usize, usize)> {
    let input = read_file("data/day04.txt")?;
    Ok((
        find_winning_score(&input, true),
        find_winning_score(&input, false),
    ))
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = read_file("data/day04-test.txt").unwrap();
        assert_eq!(find_winning_score(&input, true), 4512);
        assert_eq!(find_winning_score(&input, false), 1924);
    }

    #[test]
    fn test_my_data() {
        let input = read_file("data/day04.txt").unwrap();
        assert_eq!(find_winning_score(&input, true), 71708);
        assert_eq!(find_winning_score(&input, false), 34726);
    }
}
