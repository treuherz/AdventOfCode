use std::time::Instant;

use aoc20::util::{parse, print_answers};
use regex::Regex;

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/12")?;
    let instructions: Vec<Instruction> = inputs.iter().map(|s| s.into()).collect();
    print_answers(12, &instructions, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Debug)]
enum Instruction {
    N(i32),
    S(i32),
    E(i32),
    W(i32),
    L(i32),
    R(i32),
    F(i32),
}

impl From<&String> for Instruction {
    fn from(s: &String) -> Self {
        use Instruction::*;

        let re = Regex::new(r"^([NSEWLRF])(\d+)$").unwrap();
        let caps = re.captures(s).unwrap();
        let n: i32 = caps[2].parse().unwrap();
        match &caps[1] {
            "N" => N(n),
            "S" => S(n),
            "E" => E(n),
            "W" => W(n),
            "L" => L(n),
            "R" => R(n),
            "F" => F(n),
            _ => panic!("unrecognised instruction"),
        }
    }
}

fn part1(instructions: &[Instruction]) -> i32 {
    let mut heading = 90;
    let (mut easting, mut northing) = (0, 0);

    for i in instructions {
        match i {
            Instruction::N(d) => northing += d,
            Instruction::S(d) => northing -= d,
            Instruction::E(d) => easting += d,
            Instruction::W(d) => easting -= d,
            Instruction::L(a) => heading = ((heading - a) % 360 + 360) % 360,
            Instruction::R(a) => heading = ((heading + a) % 360 + 360) % 360,
            Instruction::F(d) => match heading {
                0 => northing += d,
                90 => easting += d,
                180 => northing -= d,
                270 => easting -= d,
                _ => panic!("non-cardinal direction"),
            },
        };
    }

    easting.abs() + northing.abs()
}

fn part2(instructions: &[Instruction]) -> i32 {
    let (mut easting, mut northing) = (0, 0);

    let (mut east_offset, mut north_offset) = (10, 1);

    for i in instructions {
        match i {
            Instruction::N(d) => north_offset += d,
            Instruction::S(d) => north_offset -= d,
            Instruction::E(d) => east_offset += d,
            Instruction::W(d) => east_offset -= d,
            Instruction::L(a) => match a {
                0 => continue,
                90 => {
                    let new_e = -north_offset;
                    let new_n = east_offset;
                    east_offset = new_e;
                    north_offset = new_n;
                }
                180 => {
                    east_offset = -east_offset;
                    north_offset = -north_offset;
                }
                270 => {
                    let new_e = north_offset;
                    let new_n = -east_offset;
                    east_offset = new_e;
                    north_offset = new_n;
                }
                _ => panic!("non-cardinal direction"),
            },
            Instruction::R(a) => match a {
                0 => continue,
                90 => {
                    let new_e = north_offset;
                    let new_n = -east_offset;
                    east_offset = new_e;
                    north_offset = new_n;
                }
                180 => {
                    east_offset = -east_offset;
                    north_offset = -north_offset;
                }
                270 => {
                    let new_e = -north_offset;
                    let new_n = east_offset;
                    east_offset = new_e;
                    north_offset = new_n;
                }
                _ => panic!("non-cardinal direction"),
            },
            Instruction::F(n) => {
                easting += east_offset * n;
                northing += north_offset * n;
            }
        };
    }

    easting.abs() + northing.abs()
}

#[cfg(test)]
mod tests {}
