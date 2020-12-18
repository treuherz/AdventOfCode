use std::{
    cmp::{max, min},
    collections::HashMap,
    collections::HashSet,
    fmt::{Display, Formatter, Write},
    ops::Not,
    time::Instant,
};

use itertools::iproduct;

use aoc20::util::{parse, print_answers};

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
    iproduct!(-1..=1, -1..=1, -1..=1, 0..=0)
        .filter(|&(x, y, z, _)| x != 0 || y != 0 || z != 0)
        .collect()
}

fn neighbours_xyzw() -> Vec<Offset> {
    iproduct!(-1..=1, -1..=1, -1..=1, -1..=1)
        .filter(|&(x, y, z, w)| x != 0 || y != 0 || z != 0 || w != 0)
        .collect()
}

struct Board {
    live: HashSet<Point>,
    counts: HashMap<Point, u32>,
    gen: u64,
    neighbourhood: Vec<Offset>,
    bbox: (Point, Point),
}

impl Board {
    pub fn new(plane: &[Vec<bool>], neighbourhood: &[Offset]) -> Board {
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
        let mut newborns: Vec<Point> = Vec::new();
        let mut deaths: Vec<Point> = Vec::new();
        for p in self.live.iter() {
            let n = self.counts.get(p).unwrap_or(&0);
            if (2..=3).contains(n).not() {
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
        const MIN_GUTTER: usize = 3;

        writeln!(f, "Generation {}", self.gen)?;

        let (a, b) = self.bbox;
        let mut x_width = (b.x - a.x + 1) as usize + MIN_GUTTER;

        for z in a.z..=b.z {
            // z,w coordinate line
            // build up list of coordinate strings and then print them all
            // padded to the same width
            let mut fragments = Vec::new();
            for w in a.w..=b.w {
                let s = format!("z={} w={}", z, w);
                x_width = max(s.len() + MIN_GUTTER, x_width);
                fragments.push(s);
            }
            for s in fragments {
                write!(f, "{:width$}", s, width = x_width)?;
            }
            writeln!(f)?;

            // x-origin line, repeated for each plane
            for _ in a.w..=b.w {
                let left_pad = (a.x - 1).abs() as usize;
                write!(f, "{:l$}{:r$}", "", '0', l = left_pad, r = x_width - left_pad)?;
            }
            writeln!(f)?;

            // the planes
            for y in a.y..=b.y {
                for w in a.w..=b.w {
                    // y-origin column
                    f.write_char(if y == 0 { '0' } else { ' ' })?;

                    for x in a.x..=b.x {
                        let p = Point { x, y, z, w };
                        f.write_char(if self.live.contains(&p) { '#' } else { '.' })?;
                    }
                    write!(f, "{:width$}", "", width = x_width - (b.x - a.x + 2) as usize)?;
                }
                writeln!(f)?;
            }
            writeln!(f)?;
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

    for _ in 1..=3 {
        println!("{}", b);
        b.step();
    }

    println!("{}", b);

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
