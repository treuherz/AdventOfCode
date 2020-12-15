use bitvec::prelude::*;
use regex::Regex;
use std::{
    collections::HashMap,
    convert::TryInto,
    ops::{BitAndAssign, BitOrAssign},
    str::{from_utf8, FromStr},
    time::Instant,
};

use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<Instruction> = parse("inputs/14")?;
    print_answers(14, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Copy, Clone, Debug)]
enum MaskValue {
    Floating,
    One,
    Zero,
}

enum Instruction {
    Mask([MaskValue; 36]),
    Mem { address: u64, value: u64 },
}

#[derive(thiserror::Error, Debug)]
#[error("{msg}")]
struct ParseError {
    msg: String,
}
impl ParseError {
    fn new(msg: &str) -> Self {
        ParseError {
            msg: msg.to_string(),
        }
    }
}

impl FromStr for Instruction {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.starts_with("mask") {
            let mask_str: &str = from_utf8(&s.as_bytes()[7..43]).unwrap();
            let mut mask = [MaskValue::Floating; 36];
            for (i, c) in mask_str.char_indices() {
                match c {
                    'X' => continue,
                    '0' => mask[36 - i - 1] = MaskValue::Zero,
                    '1' => mask[36 - i - 1] = MaskValue::One,
                    _ => return Err(ParseError::new("unrecognised mask value")),
                }
            }

            Ok(Instruction::Mask(mask))
        } else if s.starts_with("mem") {
            let re = Regex::new(r"^mem\[(\d+)\] = (\d+)$").unwrap();
            let caps = re.captures(s).unwrap();
            let address: u64 = caps[1].parse().unwrap();
            let value: u64 = caps[2].parse().unwrap();

            Ok(Instruction::Mem { address, value })
        } else {
            Err(ParseError::new("unrecognised instruction"))
        }
    }
}

fn part1(inputs: &[Instruction]) -> u128 {
    let mut map: HashMap<u64, u64> = HashMap::new();

    let mut iter = inputs.iter();

    let mut mask = if let Some(Instruction::Mask(mask)) = iter.next() {
        mask
    } else {
        panic!("first element wasn't a mask")
    };

    for inst in iter {
        match inst {
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
            Instruction::Mem { address, value } => {
                let mut bits: BitVec<Lsb0, u8> =
                    value.view_bits::<Lsb0>().iter().take(36).collect();
                bits = bits
                    .iter()
                    .zip(mask.iter())
                    .map(|(&a, &b)| {
                        match b {
                            MaskValue::Floating => {a}
                            MaskValue::One => {true}
                            MaskValue::Zero => {false}
                        }
                    })
                    .collect();
                let mut bytes: [u8; 8] = [0; 8];
                bytes
                    .iter_mut()
                    .zip(bits.as_slice())
                    .for_each(|(target, byte)| *target = *byte);
                let value = u64::from_le_bytes(bytes);
                map.insert(*address, value);
            }
        }
    }

    let mut acc = 0u128;
    map.values().for_each(|v| acc += *v as u128);
    acc
}

fn part2(inputs: &[Instruction]) -> usize {
    let mut map: HashMap<u64, u64> = HashMap::new();

    let mut iter = inputs.iter();

    let mut mask = if let Some(Instruction::Mask(mask)) = iter.next() {
        mask
    } else {
        panic!("first element wasn't a mask")
    };

    for inst in iter {
        match inst {
            Instruction::Mask(new_mask) => {
                mask = new_mask;
            }
            Instruction::Mem { address, value } => {
                let mut bits: BitVec<Lsb0, u8> =
                    address.view_bits::<Lsb0>().iter().take(36).collect();
                bits = bits
                    .iter_mut()
                    .zip(mask.iter())
                    .map(|(a, &b)| {
                        match b {
                            MaskValue::Floating => {a}
                            MaskValue::One => {true}
                            MaskValue::Zero => {false}
                        }
                    })
                    .collect();
                let mut bytes: [u8; 8] = [0; 8];
                bytes
                    .iter_mut()
                    .zip(bits.as_slice())
                    .for_each(|(target, byte)| *target = *byte);
                let value = u64::from_le_bytes(bytes);
                map.insert(*address, value);
            }
        }
    }

    let mut acc = 0u128;
    map.values().for_each(|v| acc += *v as u128);
    acc
}

#[cfg(test)]
mod tests {
    use bitvec::prelude::*;

    #[test]
    fn bit_assign() {
        let mut store = bitarr![Lsb0, u8; 0; 36];
        dbg!(store);
    }

    #[test]
    fn bits() {
        println!("{}", (2u64.pow(36)).view_bits::<Msb0>());
    }
}
