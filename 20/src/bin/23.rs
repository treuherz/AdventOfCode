use aoc20::util::print_answers;
use std::{fmt::Write, time::Instant};

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

struct Node {
    prev: usize,
    next: usize,
}

struct Ring {
    head: usize,
    arena: Vec<Node>,
}

impl Ring {
    fn new(data: &[usize], max: usize) -> Self {
        let mut arena = Vec::with_capacity(data.len().max(max) + 1);
        // Placeholder node to make indexes line up. Gross.
        arena.push(Node { prev: 0, next: 0 });
        assert!(data.len() >= 3);
        let data_max = *data.iter().max().unwrap();

        for i in 1..=max {
            if i <= data_max {
                let pos = data.iter().position(|&n| n == i).unwrap();
                let prev = data[if pos == 0 { data.len() - 1 } else { pos - 1 }];
                let next = data[if pos == data.len() - 1 { 0 } else { pos + 1 }];
                arena.push(Node { prev, next })
            } else {
                arena.push(Node {
                    prev: i - 1,
                    next: i + 1,
                })
            }
        }

        if max > data_max {
            let data_first = *data.first().unwrap();
            let data_last = *data.last().unwrap();
            let generated_first = data_max + 1;
            arena[max].next = data_first;
            arena[data_first].prev = max;
            arena[data_last].next = generated_first;
            arena[generated_first].prev = data_last;
        }

        Ring {
            arena,
            head: data[0],
        }
    }
}

fn play(mut ring: Ring, moves: i32) -> Ring {
    let min = 1;
    let max = ring.arena.len() - 1;

    for _ in 0..moves {
        let mut held = Vec::with_capacity(3);
        let mut idx = ring.arena[ring.head].next;
        for _ in 0..3 {
            held.push(idx);
            idx = ring.arena[idx].next;
        }
        ring.arena[ring.head].next = idx;

        let mut n = ring.head;
        let dst = loop {
            n = if n == min { max } else { n - 1 };
            if !held.contains(&n) {
                break n;
            }
        };

        let dst_next = ring.arena[dst].next;

        let mut idx = dst;
        for n in held {
            ring.arena[idx].next = n;
            ring.arena[n].prev = idx;
            idx = n;
        }
        ring.arena[idx].next = dst_next;
        ring.arena[dst_next].prev = idx;

        ring.head = ring.arena[ring.head].next;
    }

    ring
}

fn part1(cups: &[usize]) -> String {
    let ring = Ring::new(cups, 9);

    let ring = play(ring, 100);

    let mut out = String::new();
    let mut idx = 1;
    while ring.arena[idx].next != 1 {
        write!(out, "{}", ring.arena[idx].next).unwrap();
        idx = ring.arena[idx].next;
    }
    out
}

fn part2(cups: &[usize]) -> usize {
    let ring = Ring::new(cups, 1_000_000);

    let ring = play(ring, 10_000_000);

    let a = ring.arena[1].next;
    let b = ring.arena[a].next;
    a * b
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

    #[test]
    fn part2() {
        let cups: Vec<usize> = "389125467"
            .chars()
            .map(|c| c.to_string().parse().unwrap())
            .collect();

        assert_eq!(super::part2(&cups), 149245887792);
    }
}
