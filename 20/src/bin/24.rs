use aoc20::util::{parse, print_answers};
use nom::{branch::alt, bytes::complete::tag, combinator::map_res, multi::many1, Finish, IResult};
use std::{
    collections::{HashMap, HashSet},
    ops::Not,
    str::FromStr,
};

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/24")?;
    let initial: Vec<Vec<Step>> = inputs
        .iter()
        .map(|s| parse_steps(s).finish().unwrap().1)
        .collect();
    print_answers(24, &initial, part1, part2);
    Ok(())
}

fn parse_steps(input: &str) -> IResult<&str, Vec<Step>> {
    many1::<_, _, _, _>(map_res(
        alt((
            tag("nw"),
            tag("ne"),
            tag("sw"),
            tag("se"),
            tag("w"),
            tag("e"),
        )),
        |s: &str| s.parse::<Step>(),
    ))(input)
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
struct Point {
    x: i64,
    y: i64,
    z: i64,
}

impl Point {
    fn new() -> Point {
        Point { x: 0, y: 0, z: 0 }
    }

    fn from_steps(steps: &[Step]) -> Point {
        let mut p = Point::new();
        for step in steps {
            p = p.shift(step);
        }
        p
    }

    fn shift(&self, step: &Step) -> Point {
        let (dx, dy, dz) = match step {
            Step::NorthEast => (1, 0, -1),
            Step::East => (1, -1, 0),
            Step::SouthEast => (0, -1, 1),
            Step::SouthWest => (-1, 0, 1),
            Step::West => (-1, 1, 0),
            Step::NorthWest => (0, 1, -1),
        };
        Point {
            x: self.x + dx,
            y: self.y + dy,
            z: self.z + dz,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Step {
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    NorthWest,
}

#[derive(thiserror::Error, Debug)]
#[error("{0}")]
struct ParseError(String);

impl FromStr for Step {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Step::*;
        match s {
            "ne" => Ok(NorthEast),
            "e" => Ok(East),
            "se" => Ok(SouthEast),
            "sw" => Ok(SouthWest),
            "w" => Ok(West),
            "nw" => Ok(NorthWest),
            _ => Err(ParseError(s.to_string())),
        }
    }
}

fn neighbours<'a>() -> impl Iterator<Item = &'a Step> {
    use Step::*;
    [NorthEast, East, SouthEast, SouthWest, West, NorthWest].iter()
}

struct Board {
    live: HashSet<Point>,
    counts: HashMap<Point, u8>,
    gen: u64,
}

impl Board {
    pub fn new() -> Board {
        Board {
            live: HashSet::new(),
            counts: HashMap::new(),
            gen: 0,
        }
    }

    pub fn evolve(&mut self) {
        let mut newborns: Vec<Point> = Vec::new();
        let mut deaths: Vec<Point> = Vec::new();
        for p in self.live.iter() {
            let n = self.counts.get(p).unwrap_or(&0);
            if (1..=2).contains(n).not() {
                deaths.push(*p)
            }
        }

        for (&p, &n) in self.counts.iter() {
            if n == 2 && self.live.contains(&p).not() {
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

    fn toggle(&mut self, p: &Point) {
        if self.live.contains(p) {
            self.kill(p);
        } else {
            self.birth(p);
        }
    }

    fn kill(&mut self, p: &Point) {
        self.live.remove(&p);
        for step in neighbours() {
            self.dec(p.shift(step))
        }
    }

    fn birth(&mut self, p: &Point) {
        self.live.insert(*p);
        for step in neighbours() {
            self.inc(p.shift(step))
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

fn part1(inputs: &[Vec<Step>]) -> usize {
    let mut b = Board::new();
    for steps in inputs {
        b.toggle(&Point::from_steps(steps));
    }
    b.live_count()
}

fn part2(inputs: &[Vec<Step>]) -> usize {
    let mut b = Board::new();
    for steps in inputs {
        b.toggle(&Point::from_steps(steps));
    }
    (0..100).for_each(|_| {
        b.evolve();
    });
    b.live_count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {}

    #[test]
    fn part2() {
        let initial: Vec<Vec<Step>> = vec![
            "sesenwnenenewseeswwswswwnenewsewsw",
            "neeenesenwnwwswnenewnwwsewnenwseswesw",
            "seswneswswsenwwnwse",
            "nwnwneseeswswnenewneswwnewseswneseene",
            "swweswneswnenwsewnwneneseenw",
            "eesenwseswswnenwswnwnwsewwnwsene",
            "sewnenenenesenwsewnenwwwse",
            "wenwwweseeeweswwwnwwe",
            "wsweesenenewnwwnwsenewsenwwsesesenwne",
            "neeswseenwwswnwswswnw",
            "nenwswwsewswnenenewsenwsenwnesesenew",
            "enewnwewneswsewnwswenweswnenwsenwsw",
            "sweneswneswneneenwnewenewwneswswnese",
            "swwesenesewenwneswnwwneseswwne",
            "enesenwswwswneneswsenwnewswseenwsese",
            "wnwnesenesenenwwnenwsewesewsesesew",
            "nenewswnwewswnenesenwnesewesw",
            "eneswnwswnwsenenwnwnwwseeswneewsenese",
            "neswnwewnwnwseenwseesewsenwsweewe",
            "wseweeenwnesenwwwswnew",
        ]
        .iter()
        .map(|l| parse_steps(l).finish().unwrap().1)
        .collect();

        assert_eq!(super::part2(&initial), 2208);
    }
}
