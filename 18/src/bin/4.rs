use std::collections::hash_map::{HashMap, RandomState};
use std::error::Error;
use std::fmt;
use std::iter::Peekable;
use std::str::FromStr;

use chrono::{Duration, NaiveDateTime, Timelike};
use itertools::Itertools;
use matches::matches;
use rayon::prelude::*;
use regex::Regex;

use aoc18::util::parse;

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = parse("inputs/4")?;
    let guard_sleeps = to_sleeps(&inputs);
    let a1 = f1(&guard_sleeps);
    let a2 = f2(&guard_sleeps);
    println!("{}\n{}", a1, a2);
    Ok(())
}

fn f1(sleeps: &HashMap<usize, Vec<Sleep>, RandomState>) -> usize {
    let (sleepiest_id, sleepiest_sleeps) = *sleeps
        .iter()
        .sorted_by_key(|(_, sleeps)| sleeps.iter().map(|s| s.len().num_minutes()).sum::<i64>())
        .last()
        .unwrap();
    let sleep_mins = sleeps_per_minute(sleepiest_sleeps);
    let (sleepiest_minute, _) = sleep_mins
        .iter()
        .enumerate()
        .max_by_key(|(_, &slept)| slept)
        .unwrap();
    *sleepiest_id * sleepiest_minute
}

fn f2(sleeps: &HashMap<usize, Vec<Sleep>, RandomState>) -> usize {
    let guards_minutes_most_asleep = sleeps
        .iter()
        .map(|(id, sleeps)| (id, sleeps_per_minute(sleeps)))
        .map(|(id, mins)| {
            let (ref minute, times_asleep) = mins
                .iter()
                .enumerate()
                .max_by_key(|(_, &slept)| slept)
                .unwrap();
            (id, (*minute, *times_asleep))
        });
    let (mut max_sleeps_id, mut max_sleeps_minute, mut max_sleeps) = (0usize, 0usize, 0usize);
    for (id, (minute, times_asleep)) in guards_minutes_most_asleep {
        if times_asleep > max_sleeps {
            max_sleeps_id = *id;
            max_sleeps_minute = minute;
            max_sleeps = times_asleep;
        }
    }
    max_sleeps_id * max_sleeps_minute
}

fn to_sleeps(inputs: &[String]) -> HashMap<usize, Vec<Sleep>, RandomState> {
    let records = inputs
        .par_iter()
        .map(|i| i.parse::<Record>().unwrap())
        .collect::<Vec<Record>>()
        .into_iter()
        .sorted_by_key(|&r| match r {
            Record::Guard { ts, .. } => ts,
            Record::Sleep(ts) => ts,
            Record::Wake(ts) => ts,
        });
    let shifts = records.into_iter().shifts();
    let mut guard_sleeps = HashMap::new();
    shifts.for_each(|mut s: Shift| {
        guard_sleeps
            .entry(s.guard_id)
            .or_insert_with(Vec::new)
            .append(&mut s.sleeps)
    });
    guard_sleeps
}

fn sleeps_per_minute(sleeps: &[Sleep]) -> Vec<usize> {
    let mut sleep_mins = vec![0usize; 60];
    for sleep in sleeps.iter() {
        for minute in sleep.start.minute()..sleep.end.minute() {
            sleep_mins[(minute as usize)] += 1;
        }
    }
    sleep_mins
}

#[derive(Debug, Clone, Copy)]
enum Record {
    Guard { ts: NaiveDateTime, id: usize },
    Sleep(NaiveDateTime),
    Wake(NaiveDateTime),
}

impl FromStr for Record {
    type Err = ParseRecordError;

    fn from_str(s: &str) -> Result<Self, <Self as FromStr>::Err> {
        let ts_re = Regex::new(r"^\[(\d{4}-\d{2}-\d{2} \d{2}:\d{2})\]").unwrap();
        let guard_re = Regex::new(r"Guard #(\d+) begins shift$").unwrap();
        let sleep_ending = "falls asleep";
        let wake_ending = "wakes up";

        let ts = ts_re
            .captures(&s[..19])
            .ok_or_else(|| ParseRecordError::new(s, "TS did not match regex"))?
            .get(1)
            .map(|m| NaiveDateTime::parse_from_str(m.as_str(), "%Y-%m-%d %H:%M"))
            .unwrap()
            .map_err(|_| ParseRecordError::new(s, "TS could not be parsed"))?;

        let message = &s[19..];
        if let Some(caps) = guard_re.captures(message) {
            let guard_id = caps.get(1).unwrap().as_str().parse();
            match guard_id {
                Ok(id) => Ok(Record::Guard { ts, id }),
                Err(_) => Err(ParseRecordError::new(s, "Guard ID could not be parsed")),
            }
        } else if message.ends_with(sleep_ending) {
            Ok(Record::Sleep(ts))
        } else if message.ends_with(wake_ending) {
            Ok(Record::Wake(ts))
        } else {
            Err(ParseRecordError::new(s, "Message didn't match any pattern"))
        }
    }
}

#[derive(Debug)]
struct Shift {
    guard_id: usize,
    sleeps: Vec<Sleep>,
}

#[derive(Debug)]
struct Sleep {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl Sleep {
    fn len(&self) -> Duration {
        self.end - self.start
    }
}

struct ShiftParser<I: Iterator> {
    iter: Peekable<I>,
}

impl<I: Iterator> Iterator for ShiftParser<I>
where
    I: Iterator<Item = Record>,
{
    type Item = Shift;

    fn next(&mut self) -> Option<<Self as Iterator>::Item> {
        let guard_rec: Record = self.iter.next()?;
        let (mut shift, mut cur_start) = match guard_rec {
            Record::Guard { ts, id } => (
                Shift {
                    guard_id: id,
                    sleeps: Vec::new(),
                },
                ts,
            ),
            _ => panic!("Iterator must start with a Guard record!"),
        };
        let mut push_sleep = |start, end| shift.sleeps.push(Sleep { start, end });
        for rec in self
            .iter
            .peeking_take_while(|r| !matches!(r, Record::Guard{..}))
        {
            match rec {
                Record::Sleep(ts) => cur_start = ts,
                Record::Wake(ts) => push_sleep(cur_start, ts),
                Record::Guard { .. } => unreachable!(),
            }
        }
        Some(shift)
    }
}

trait ShiftParserExt: Iterator {
    fn shifts(self) -> ShiftParser<Self>
    where
        Self: Sized,
    {
        ShiftParser {
            iter: self.peekable(),
        }
    }
}

impl<I: Iterator> ShiftParserExt for I {}

#[derive(Debug)]
struct ParseRecordError {
    string: String,
    message: String,
}

impl ParseRecordError {
    fn new(s: &str, m: &str) -> ParseRecordError {
        ParseRecordError {
            string: String::from(s),
            message: String::from(m),
        }
    }
}

impl<'a> fmt::Display for ParseRecordError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Error parsing string {} into record: {}",
            self.string, self.message
        )
    }
}

impl<'a> Error for ParseRecordError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}
