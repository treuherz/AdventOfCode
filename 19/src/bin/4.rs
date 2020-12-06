use aoc19::util::print_answers;
use std::cmp::Ordering;

fn main() -> anyhow::Result<()> {
    let inputs: Vec<u64> = (372304..847060).collect();
    print_answers(4, &inputs, f1, f2);
    Ok(())
}

fn f1(range: &[u64]) -> usize {
    let filtered: Vec<&u64> = range.iter().filter(|&n| pred1(n)).collect();
    filtered.len()
}

fn f2(range: &[u64]) -> usize {
    let filtered: Vec<&u64> = range.iter().filter(|&n| pred2(n)).collect();
    filtered.len()
}

fn pred1(n: &u64) -> bool {
    let mut has_double = false;
    let mut ever_ascends = false;

    let s = n.to_string();
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    for cur in chars {
        match prev.cmp(&cur) {
            Ordering::Equal => has_double = true,
            Ordering::Greater => ever_ascends = true,
            Ordering::Less => {}
        }
        prev = cur;
    }
    return has_double && !ever_ascends;
}

fn pred2(n: &u64) -> bool {
    let mut doubles = 0;
    let mut currently_double = false;
    let mut currently_overlong = false;
    let mut ever_ascends = false;

    let s = n.to_string();
    let mut chars = s.chars();
    let mut prev = chars.next().unwrap();
    for cur in chars {
        match prev.cmp(&cur) {
            Ordering::Equal => match (currently_double, currently_overlong) {
                (false, _) => {
                    doubles += 1;
                    currently_double = true;
                }
                (true, false) => {
                    doubles -= 1;
                    currently_overlong = true;
                }
                (true, true) => {}
            },
            Ordering::Greater => {
                ever_ascends = true;
                currently_double = false;
                currently_overlong = false;
            }
            Ordering::Less => {
                currently_double = false;
                currently_overlong = false;
            }
        }
        prev = cur;
    }
    return doubles > 0 && !ever_ascends;
}
