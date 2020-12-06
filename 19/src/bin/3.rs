use std::collections::HashMap;
use std::convert::TryInto;
use std::str::FromStr;

use anyhow::anyhow;

use aoc19::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let inputs: Vec<Wire> = parse_wires(parse("inputs/3")?);
    print_answers(3, &inputs, f1, f2);
    Ok(())
}

fn f1(wires: &[Wire]) -> u64 {
    let mut grid: Grid<bool> = Grid::new();
    let mut crossings: Vec<Coord> = Vec::new();
    for (idx, wire) in wires.iter().enumerate() {
        let mut pos = Coord(0, 0);
        for seg in wire.iter() {
            for _ in 1..=seg.distance {
                pos.shift(&seg.direction);
                let m = grid.insert_and_get(&pos, idx, true);
                if m.len() > 1 {
                    crossings.push(pos)
                }
            }
        }
    }
    crossings.iter().map(Coord::dist).min().unwrap()
}

fn f2(wires: &[Wire]) -> u64 {
    let mut grid: Grid<u64> = Grid::new();
    let mut crossings: Vec<u64> = Vec::new();
    for (idx, wire) in wires.iter().enumerate() {
        let mut pos = Coord(0, 0);
        let mut count = 0;
        for seg in wire.iter() {
            for _ in 1..=seg.distance {
                count += 1;
                pos.shift(&seg.direction);
                let m = grid.insert_and_get(&pos, idx, count);
                if m.len() > 1 {
                    crossings.push(m.values().sum())
                }
            }
        }
    }
    *crossings.iter().min().unwrap()
}

fn parse_wires(input: Vec<String>) -> Vec<Wire> {
    input
        .iter()
        .map(|i| {
            i.split(',')
                .map(|s| s.parse())
                .collect::<anyhow::Result<Wire>>()
                .unwrap()
        })
        .collect()
}

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
struct Coord(i64, i64);

impl Coord {
    fn shift(&mut self, d: &Direction) {
        match d {
            Direction::Up => self.1 += 1,
            Direction::Down => self.1 -= 1,
            Direction::Left => self.0 += 1,
            Direction::Right => self.0 -= 1,
        };
    }

    fn dist(&self) -> u64 {
        (self.0.abs() + self.1.abs()).try_into().unwrap()
    }
}

struct Grid<T> {
    storage: HashMap<Coord, HashMap<usize, T>>,
}

impl<T> Grid<T> {
    fn new() -> Grid<T> {
        Grid {
            storage: HashMap::new(),
        }
    }

    fn insert_and_get(&mut self, coord: &Coord, idx: usize, v: T) -> &HashMap<usize, T> {
        match self.storage.get_mut(coord) {
            Some(m) => {
                if !m.contains_key(&idx) {
                    m.insert(idx, v);
                }
            }
            None => {
                let mut m = HashMap::new();
                m.insert(idx, v);
                self.storage.insert(*coord, m);
            }
        }
        &self.storage.get(coord).unwrap()
    }
}

type Wire = Vec<Segment>;

#[derive(Debug)]
struct Segment {
    direction: Direction,
    distance: u64,
}

impl FromStr for Segment {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Direction::*;

        let mut chars = s.chars();
        let direction = match chars.next().unwrap() {
            'U' => Up,
            'D' => Down,
            'L' => Left,
            'R' => Right,
            l @ _ => return Err(anyhow!("{} is not a direction", l)),
        };
        let distance: u64 = chars.as_str().parse()?;
        Ok(Segment {
            direction,
            distance,
        })
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}
