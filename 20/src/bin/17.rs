use std::collections::HashSet;
use std::{collections::HashMap, time::Instant};

use lazy_static::lazy_static;

use aoc20::util::{parse, print_answers};
use nom::lib::std::fmt::Formatter;
use std::cmp::{max, min};
use std::fmt::{Display, Write};
use std::ops::Not;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/17")?;
    print_answers(17, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
    w: i64,
}

impl Point {
    fn new() -> Point {
        Point {
            x: 0,
            y: 0,
            z: 0,
            w: 0,
        }
    }

    fn shift(&self, x: i64, y: i64, z: i64, w: i64) -> Point {
        Point {
            x: self.x + x,
            y: self.y + y,
            z: self.z + z,
            w: self.w + w,
        }
    }
}

type Offset = (i64, i64, i64, i64);

fn neighbours_xyz() -> Vec<Offset> {
    let mut v: Vec<Offset> = Vec::with_capacity(3usize.pow(3) - 1);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                if x == 0 && y == 0 && z == 0 {
                    continue;
                }
                v.push((x, y, z, 0));
            }
        }
    }
    v
}

fn neighbours_xyzw() -> Vec<Offset> {
    let mut v: Vec<Offset> = Vec::with_capacity(3usize.pow(4) - 1);
    for x in -1..=1 {
        for y in -1..=1 {
            for z in -1..=1 {
                for w in -1..=1 {
                    if x == 0 && y == 0 && z == 0 && w == 0 {
                        continue;
                    }
                    v.push((x, y, z, w));
                }
            }
        }
    }
    v
}

struct Board {
    live: HashSet<Point>,
    counts: HashMap<Point, u32>,
    gen: u64,
    neighbourhood: Vec<Offset>,
    bbox: (Point, Point),
}

impl Board {
    pub fn new(plane: &Vec<Vec<bool>>, neighbourhood: &[Offset]) -> Board {
        let mut b = Board {
            live: HashSet::new(),
            counts: HashMap::new(),
            neighbourhood: neighbourhood.to_owned(),
            gen: 0,
            bbox: (Point::new(), Point::new()),
        };
        for (y, row) in plane.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if *cell {
                    b.birth(&Point {
                        x: x as i64,
                        y: y as i64,
                        z: 0,
                        w: 0,
                    });
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
            if n < 2 || 3 < n {
                deaths.push(*p)
            }
        }

        for (&p, &n) in self.counts.iter() {
            if n == 3 && self.live.contains(&p).not() {
                newborns.push(p)
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
        for &(x, y, z, w) in self.neighbourhood.clone().iter() {
            self.dec(p.shift(x, y, z, w))
        }
    }

    fn birth(&mut self, p: &Point) {
        let (a, b) = &mut self.bbox;
        if p.x < a.x || p.y < a.y || p.z < a.z || p.w < a.w {
            *a = Point {
                x: min(p.x, a.x),
                y: min(p.y, a.y),
                z: min(p.z, a.x),
                w: min(p.w, a.w),
            }
        }
        if p.x > b.x || p.y > b.y || p.z > b.z || p.w < a.w {
            *b = Point {
                x: max(p.x, b.x),
                y: max(p.y, b.y),
                z: max(p.z, b.z),
                w: max(p.w, b.w),
            }
        }

        self.live.insert(*p);
        for &(x, y, z, w) in self.neighbourhood.clone().iter() {
            self.inc(p.shift(x, y, z, w))
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
        for z in a.z..=b.z {
            write!(f, "z = {}\n", z)?;
            for x in a.x - 1..=b.x {
                let c = if x == 0 { '0' } else { ' ' };
                f.write_char(c)?;
            }
            f.write_char('\n')?;
            for y in a.y..=b.y {
                let c = if y == 0 { '0' } else { ' ' };
                f.write_char(c)?;
                for x in a.x..=b.x {
                    let p = Point { x: x, y: y, z: z, w: 0 };
                    let c = if self.live.contains(&p) { '#' } else { '.' };
                    f.write_char(c)?;
                }
                f.write_char('\n')?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn parse_plane(inputs: &[String]) -> Vec<Vec<bool>> {
    let plane: Vec<Vec<bool>> = inputs
        .iter()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    '.' => false,
                    '#' => true,
                    _ => panic!("unrecognised character"),
                })
                .collect()
        })
        .collect();
    plane
}

fn part1(inputs: &[String]) -> usize {
    let plane = parse_plane(inputs);

    let mut b = Board::new(&plane, &neighbours_xyz());

    for _ in 1..=6 {
        // println!("{}", b);
        b.step();
    }

    b.live_count()
}

fn part2(inputs: &[String]) -> usize {
    let plane = parse_plane(inputs);

    let mut b = Board::new(&plane, &neighbours_xyzw());

    for _ in 1..=6 {
        // println!("{}", b);
        b.step();
    }

    b.live_count()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let input = [".#.".to_string(), "..#".to_string(), "###".to_string()];

        assert_eq!(super::part1(&input), 112);
    }

    #[test]
    fn part2() {
        let input = [".#.".to_string(), "..#".to_string(), "###".to_string()];

        assert_eq!(super::part2(&input), 848);
    }
}
