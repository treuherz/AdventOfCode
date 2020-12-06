#![feature(str_split_once)]
#![feature(or_patterns)]

use anyhow::{anyhow, Context};
use aoc20::util::{parse, print_answers};
use lazy_static::lazy_static;
use regex::Regex;
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/4")?;
    let passports = parse_passports(&inputs);
    print_answers(4, &passports, f1, f2);
    Ok(())
}

fn parse_passports(input: &Vec<String>) -> Vec<Passport> {
    let mut passports: Vec<Passport> = vec![];
    let mut cur: Passport = Passport(HashMap::new());
    for s in input {
        if s.is_empty() {
            passports.push(cur);
            cur = Passport(HashMap::new());
            continue;
        }
        s.split(" ").for_each(|kv| {
            let (k, v) = kv.split_once(":").unwrap();
            cur.0.insert(k, v);
        })
    }
    passports.push(cur);

    passports
}


lazy_static! {
    static ref REQUIRED_KEYS: Vec<&'static str> =
        vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
    static ref VALID_ECLS: Vec<&'static str> =
        vec!["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];
    static ref HCL_REGEX: Regex = Regex::new(r"^#[a-f0-9]{6}$").unwrap();
    static ref PID_REGEX: Regex = Regex::new(r"^[0-9]{9}$").unwrap();
    static ref HGT_REGEX: Regex = Regex::new(r"^(?P<num>\d+)(?P<unit>(in|cm))$").unwrap();
}

struct Passport<'a>(HashMap<&'a str, &'a str>);

impl<'a> Passport<'_> {
    fn has_required_keys(&self) -> bool {
        REQUIRED_KEYS.iter().all(|&k| self.0.contains_key(k))
    }

    fn is_valid(&self) -> bool {
        self.has_required_keys()
            && self.0.iter().enumerate().all(|(n, (&k, &v))| {
                let res = Passport::validate_field(k, v);
                if let Err(e) = &res {
                    dbg!(&e);
                }
                res.is_ok()
            })
    }

    fn validate_field(k: &str, v: &str) -> anyhow::Result<()> {
        match k {
            "byr" | "iyr" | "eyr" => match (k, v.parse::<u64>()) {
                ("byr", Ok(1920..=2002)) => Ok(()),
                ("iyr", Ok(2010..=2020)) => Ok(()),
                ("eyr", Ok(2020..=2030)) => Ok(()),
                (_, Ok(y)) => Err(anyhow!("out of range {}, {}", k, y)),
                (_, Err(e)) => Err(e).with_context(|| format!("unparseable {}, {}", k, v)),
            },
            "hgt" => match HGT_REGEX.captures(v) {
                None => Err(anyhow!("invalid height {}", v)),
                Some(caps) => {
                    let num: u64 = caps.name("num").unwrap().as_str().parse().unwrap();
                    let unit = caps.name("unit").unwrap().as_str();
                    match (num, unit) {
                        (150..=193, "cm") => Ok(()),
                        (59..=76, "in") => Ok(()),
                        (_, _) => Err(anyhow!("out of range hgt {}", v)),
                    }
                }
            },
            "ecl" => match VALID_ECLS.contains(&v) {
                true => Ok(()),
                false => Err(anyhow!("invalid ecl, {}", v)),
            },
            "hcl" => match HCL_REGEX.is_match(v) {
                true => Ok(()),
                false => Err(anyhow!("invalid hcl, {}", v)),
            },
            "pid" => match PID_REGEX.is_match(v) {
                true => Ok(()),
                false => Err(anyhow!("invalid pid, {}", v)),
            },
            "cid" => Ok(()),
            _ => Err(anyhow!("unknown key")),
        }
    }
}

fn f1(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|&p| p.has_required_keys()).count()
}

fn f2(passports: &Vec<Passport>) -> usize {
    passports.iter().filter(|&p| p.is_valid()).count()
}
