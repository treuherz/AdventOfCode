use aoc20::util::print_answers;
use itertools::Itertools;
use std::{collections::VecDeque, time::Instant};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let cups: Vec<usize> = "368195742"
        .chars()
        .map(|c| c.to_string().parse().unwrap())
        .collect();
    print_answers(23, &cups, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

fn play(mut ring: VecDeque<usize>, moves: i32) -> VecDeque<usize> {
    let min = *ring.iter().min().unwrap();
    let max = *ring.iter().max().unwrap();

    ring.rotate_left(1);

    let mut t = Instant::now();
    let mut tt = Instant::now();
    for i in 0..moves {
        if i % 10_000 == 0 {
            dbg!(i, t.elapsed());
            t = Instant::now();
            tt = Instant::now();
        };

        let mut held = VecDeque::with_capacity(3);
        (0..3).for_each(|_| held.push_back(ring.pop_front().unwrap()));

        let cur = *ring.back().unwrap();
        let mut dst = if cur == min { max } else { cur - 1 };

        let dst_pos = loop {
            if held.contains(&dst) {
                if dst == min {
                    dst = max;
                } else {
                    dst -= 1;
                }
            } else {
                if i % 10_000 == 0 {
                    dbg!(tt.elapsed());
                    tt = Instant::now();
                };
                break ring.iter().rposition(|&n| n == dst).unwrap();
            }
        };
        if i % 10_000 == 0 {
            dbg!(dst_pos, tt.elapsed());
            tt = Instant::now();
        };

        ring.rotate_left(dst_pos+1);
        (0..3).for_each(|_| ring.push_front(held.pop_back().unwrap()));
        ring.rotate_right(dst_pos);
        if i % 10_000 == 0 {
            dbg!(tt.elapsed());
            tt = Instant::now();
        };
    }

    ring
}

fn part1(cups: &[usize]) -> String {
    let ring: VecDeque<usize> = cups.iter().copied().collect();

    let mut ring = play(ring, 100);

    while ring[0] != 1 {
        ring.rotate_right(1);
    }
    ring.iter().skip(1).map(|n| n.to_string()).join("")
}

fn part2(cups: &[usize]) -> usize {
    let max = *cups.iter().max().unwrap();
    let ring: VecDeque<usize> = cups.iter().copied().chain((max + 1)..=1_000_000).collect();

    let mut ring = play(ring, 10_000_000);

    while ring[0] != 1 {
        ring.rotate_right(1);
    }
    ring.iter().skip(1).take(2).product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        let cups: Vec<usize> = "389125467"
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        assert_eq!(super::part1(&cups), "67384529".to_string());
    }
}
