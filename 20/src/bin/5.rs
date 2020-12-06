use aoc20::util::{parse, print_answers};
use itertools::Itertools;
use std::ops::Range;

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/5")?;
    let ids = parse_ids(&inputs);
    print_answers(5, &ids, f1, f2);
    Ok(())
}

const ROWS: Range<u32> = 0..127;
const COLS: Range<u32> = 0..7;

fn len(pair: (u32, u32)) -> u32 {
    pair.1 - pair.0
}

fn parse_ids(inputs: &[String]) -> Vec<u32> {
    inputs
        .iter()
        // Break row and column spec apart
        .map(|s| s.split_at(7))
        // Follow instructions
        .map(|(fb, lr)| {
            let row_pair = fb.chars().fold((ROWS.start, ROWS.end), |pair, c| match c {
                // First half if F, second half if B
                'F' => (pair.0, pair.0 + len(pair) / 2),
                'B' => (pair.0 + len(pair) / 2 + 1, pair.1),
                _ => panic!("expected 'F' or 'B'"),
            });

            // Make sure we've narrowed it down to a single number
            assert_eq!(row_pair.0, row_pair.1);

            let col_pair = lr.chars().fold((COLS.start, COLS.end), |pair, c| match c {
                // First half if L, second half if R
                'L' => (pair.0, pair.0 + len(pair) / 2),
                'R' => (pair.0 + len(pair) / 2 + 1, pair.1),
                _ => panic!("expected 'R' or 'L'"),
            });

            // Make sure we've narrowed it down to a single number
            assert_eq!(col_pair.0, col_pair.1);

            (row_pair.0, col_pair.0)
        })
        // Get the ID from the coordinate
        .map(|(row, col)| row * 8 + col)
        .collect()
}

fn f1(ids: &[u32]) -> u32 {
    // Greatest ID number
    *ids.iter().max().unwrap()
}

fn f2(ids: &[u32]) -> u32 {
    // Find missing ID number. First we sort, then we check for a pair of non-consecutive IDs
    let mut ids = ids.to_owned();
    ids.sort_unstable();
    ids.iter()
        .tuple_windows()
        .find(|(&m, &n)| m + 1 != n)
        .unwrap().0 + 1
}
