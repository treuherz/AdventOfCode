use std::{
    collections::HashMap,
    time::Instant,
};
use std::collections::HashSet;

use lazy_static::lazy_static;

use aoc20::util::{parse, print_answers};
use std::fmt::{Display, Write};
use nom::lib::std::fmt::Formatter;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/17")?;
    print_answers(17, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point(i64, i64, i64);

type Transform = fn(&Point) -> Point;
lazy_static! {
    static ref NEIGHBOURHOOD: Vec<Transform>  = vec![
        |Point(x, y, z)| Point(*y - 1, *x - 1, *z - 1),
        |Point(x, y, z)| Point(*y - 1, *x, *z - 1),
        |Point(x, y, z)| Point(*y - 1, *x + 1, *z - 1),
        |Point(x, y, z)| Point(*y, *x + 1, *z - 1),
        |Point(x, y, z)| Point(*y + 1, *x + 1, *z - 1),
        |Point(x, y, z)| Point(*y + 1, *x, *z - 1),
        |Point(x, y, z)| Point(*y + 1, *x - 1, *z - 1),
        |Point(x, y, z)| Point(*y, *x - 1, *z - 1),
        |Point(x, y, z)| Point(*y, *x, *z - 1),
        |Point(x, y, z)| Point(*y - 1, *x - 1, *z),
        |Point(x, y, z)| Point(*y - 1, *x, *z),
        |Point(x, y, z)| Point(*y - 1, *x + 1, *z),
        |Point(x, y, z)| Point(*y, *x + 1, *z),
        |Point(x, y, z)| Point(*y + 1, *x + 1, *z),
        |Point(x, y, z)| Point(*y + 1, *x, *z),
        |Point(x, y, z)| Point(*y + 1, *x - 1, *z),
        |Point(x, y, z)| Point(*y, *x - 1, *z),
        |Point(x, y, z)| Point(*y - 1, *x - 1, *z + 1),
        |Point(x, y, z)| Point(*y - 1, *x, *z + 1),
        |Point(x, y, z)| Point(*y - 1, *x + 1, *z + 1),
        |Point(x, y, z)| Point(*y, *x + 1, *z + 1),
        |Point(x, y, z)| Point(*y + 1, *x + 1, *z + 1),
        |Point(x, y, z)| Point(*y + 1, *x, *z + 1),
        |Point(x, y, z)| Point(*y + 1, *x - 1, *z + 1),
        |Point(x, y, z)| Point(*y, *x - 1, *z + 1),
        |Point(x, y, z)| Point(*y, *x, *z + 1),
    ];
}

struct Board {
    live: HashSet<Point>,
    counts: HashMap<Point, u32>,
    gen: u64,
    bbox: (Point, Point),
}

impl Board {
    pub fn new(plane: &Vec<Vec<bool>>) -> Board {
        let mut b = Board {
            live: HashSet::new(),
            counts: HashMap::new(),
            gen: 0,
            bbox: (Point(0, 0, 0), Point(0, 0, 0)),
        };
        for (y, row) in plane.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    b.birth(&Point(x as i64, y as i64, 0));
                }
            }
        }
        b
    }

    pub fn step(&mut self) {
        dbg!(self.gen);
        let mut newborns: Vec<Point> = Vec::new();
        let mut deaths: Vec<Point> = Vec::new();
        for p in self.live.iter() {
            let n = *self.counts.get(p).unwrap_or(&0);
            if n < 2 || n > 3 {
                deaths.push(*p)
            }
        }

        for (p, n) in self.counts.iter() {
            if *n == 3 && !self.live.contains(p) {
                newborns.push(*p)
            }
        }

        for p in newborns {
            self.birth(&p)
        }
        for p in deaths {
            self.kill(&p)
        }

        self.gen += 1;
    }

    fn kill(&mut self, p: &Point) {
        self.live.remove(&p);
        for f in NEIGHBOURHOOD.iter() {
            self.dec(f(&p))
        }
    }

    fn birth(&mut self, p: &Point) {
        let (a, b) = &mut self.bbox;
        if p.0 < a.0 || p.1 < a.1 || p.2 < a.2 {
            *a = *p
        }
        if p.0 > b.0 || p.1 > b.1 || p.2 > b.2 {
            *b = *p
        }

        self.live.insert(*p);
        for f in NEIGHBOURHOOD.iter() {
            self.inc(f(&p))
        }
    }

    fn inc(&mut self, p: Point) {
        if let Some(n) = self.counts.get_mut(&p) {
            *n += 1;
        } else {
            self.counts.insert(p, 1);
        }
    }

    fn dec(&mut self, p: Point) {
        if let Some(n) = self.counts.get_mut(&p) {
            if *n == 1 {
                // Don't track cells with no neighbours
                self.counts.remove(&p);
                return;
            }

            *n -= 1;
        }
    }

    pub fn live_count(&self) -> usize {
        self.live.len()
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let (a, b) = self.bbox;
        for z in (a.2..=b.2) {
            for y in (a.1..=b.1) {
                for x in (a.0..=b.0) {
                    if self.live.contains(&Point(x, y, z)) {
                        f.write_char('#')?;
                    } else {
                        f.write_char('.')?;
                    }
                }
                f.write_char('\n')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn part1(inputs: &[String]) -> usize {
    let plane: Vec<Vec<bool>> = inputs.iter().map(
        |s| s.chars().map(|c|
            match c {
                '.' => false,
                '#' => true,
                _ => panic!("unrecognised character")
            }).collect()
    ).collect();

    let mut b = Board::new(&plane);

    for _ in 1..=6 {
        println!("Generation #{}", b.gen);
        println!("{}", b);
        b.step();
    }

    b.live_count()
}

fn part2(inputs: &[String]) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = [
            ".#.".to_string(),
            "..#".to_string(),
            "###".to_string(),
        ];

        assert_eq!(super::part1(&input), 112);
    }
}
