use std::time::Instant;

use aoc20::util::{parse, print_answers};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::ops::Index;
use std::ops::IndexMut;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/11")?;
    let pattern: Pattern = inputs.into();
    print_answers(11, &pattern, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Floor,
    Empty,
    Occupied,
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        use Cell::*;
        match c {
            '.' => Floor,
            'L' => Empty,
            '#' => Occupied,
            _ => panic!("unrecognised cell"),
        }
    }
}

impl From<Vec<String>> for Pattern {
    fn from(v: Vec<String>) -> Self {
        Pattern(
            v.iter()
                .map(|s| s.chars().map(|c| c.into()).collect())
                .collect(),
        )
    }
}

#[derive(Clone, Hash)]
struct Pattern(Vec<Vec<Cell>>);

impl Index<(usize, usize)> for Pattern {
    type Output = Cell;

    fn index(&self, (y, x): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl IndexMut<(usize, usize)> for Pattern {
    fn index_mut(&mut self, (y, x): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

impl Pattern {
    fn count_occupied(&self, (y, x): (usize, usize)) -> usize {
        let (y, x): (isize, isize) = (y as isize, x as isize);
        let (len_y, len_x) = self.dim();

        let neighbourhood: Vec<(isize, isize)> = vec![
            (y - 1, x - 1),
            (y - 1, x),
            (y - 1, x + 1),
            (y, x + 1),
            (y + 1, x + 1),
            (y + 1, x),
            (y + 1, x - 1),
            (y, x - 1),
        ];

        neighbourhood
            .iter()
            // Remove nonexistent neighbours
            .filter(|(y, x)| !(*y < 0 || *x < 0 || *y as usize >= len_y || *x as usize >= len_x))
            .map(|(y, x)| self[(*y as usize, *x as usize)])
            .filter(|&n| n == Cell::Occupied)
            .count()
    }


    fn count_occupied_distant(&self, (y, x): (usize, usize)) -> usize {
        let (y, x): (isize, isize) = (y as isize, x as isize);
        let (len_y, len_x) = self.dim();

        type Transform = fn(isize, isize) -> (isize, isize);
        let neighbourhood: Vec<Transform>  = vec![
            |y, x| (y - 1, x - 1),
            |y, x| (y - 1, x),
            |y, x| (y - 1, x + 1),
            |y, x| (y, x + 1),
            |y, x| (y + 1, x + 1),
            |y, x| (y + 1, x),
            |y, x| (y + 1, x - 1),
            |y, x| (y, x - 1),
        ];

        let mut acc = 0;
        for f in neighbourhood {
            let (mut y, mut x): (isize, isize) = (y, x);
            acc += loop {
                let idx = f(y, x);
                y = idx.0;
                x = idx.1;
                if y < 0 || x < 0 || y as usize >= len_y || x as usize >= len_x {
                    break 0;
                }
                match self[(y as usize, x as usize)] {
                    Cell::Empty => {
                        break 0;
                    }
                    Cell::Occupied => {
                        break 1;
                    }
                    Cell::Floor => {
                        continue;
                    }
                }
            }
        }

        acc
    }

    fn advance(&self, threshold: usize) -> Pattern {
        let mut next: Pattern = self.clone();

        for (i, row) in self.0.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if Cell::Empty == c && self.count_occupied((i, j)) == 0 {
                    next[(i, j)] = Cell::Occupied;
                }
                if Cell::Occupied == c && self.count_occupied((i, j)) >= threshold {
                    next[(i, j)] = Cell::Empty;
                }
            }
        }

        next
    }

    fn advance_distant(&self, threshold: usize) -> Pattern {
        let mut next: Pattern = self.clone();

        for (i, row) in self.0.iter().enumerate() {
            for (j, &c) in row.iter().enumerate() {
                if Cell::Empty == c && self.count_occupied_distant((i, j)) == 0 {
                    next[(i, j)] = Cell::Occupied;
                }
                if Cell::Occupied == c && self.count_occupied_distant((i, j)) >= threshold {
                    next[(i, j)] = Cell::Empty;
                }
            }
        }

        next
    }

    fn dim(&self) -> (usize, usize) {
        (self.0.len(), self.0[0].len())
    }
}

fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

fn part1(pattern: &Pattern) -> usize {
    let mut cur = pattern.to_owned();
    let pat = loop {
        let next = cur.advance(4);
        if calculate_hash(&next) == calculate_hash(&cur) {
            break next;
        }
        cur = next;
    };
    pat.0
        .iter()
        .flatten()
        .filter(|&&c| c == Cell::Occupied)
        .count()
}

fn part2(pattern: &Pattern) -> usize {
    let mut cur = pattern.to_owned();
    let pat = loop {
        let next = cur.advance_distant(5);
        if calculate_hash(&next) == calculate_hash(&cur) {
            break next;
        }
        cur = next;
    };
    pat.0
        .iter()
        .flatten()
        .filter(|&&c| c == Cell::Occupied)
        .count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let pat: super::Pattern = input.into();

        assert_eq!(super::part1(&pat), 37);
    }

    #[test]
    fn part2() {
        let input: Vec<String> = vec![
            "L.LL.LL.LL",
            "LLLLLLL.LL",
            "L.L.L..L..",
            "LLLL.LL.LL",
            "L.LL.LL.LL",
            "L.LLLLL.LL",
            "..L.L.....",
            "LLLLLLLLLL",
            "L.LLLLLL.L",
            "L.LLLLL.LL",
        ]
        .iter()
        .map(|s| s.to_string())
        .collect();

        let pat: super::Pattern = input.into();

        assert_eq!(super::part2(&pat), 26);
    }
}
