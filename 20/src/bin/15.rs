use std::{
    collections::HashMap,
    time::Instant,
};

use aoc20::util::print_answers;

fn main() {
    let now = Instant::now();
    let inputs: Vec<usize> = "5,2,8,16,18,0,1".split(',').map(|s| s.parse().unwrap()).collect();
    print_answers(15, &inputs, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
}

struct Game {
    memory: HashMap<usize, usize>,
    t: usize,
    prev: Option<usize>,
    inputs: Vec<usize>,
}

impl Game {
    fn new(inputs: &[usize]) -> Game {
        Game {
            memory: HashMap::new(),
            t: 0,
            prev: None,
            inputs: inputs.to_vec(),
        }
    }
}

impl Iterator for Game {
    type Item = usize;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let speak = {
            if self.t < self.inputs.len() {
                self.inputs[self.t]
            } else if let Some(prev_t) = self.memory.get(&self.prev.unwrap()) {
                self.t - 1 - prev_t
            } else {
                0
            }
        };

        if let Some(n) = self.prev {
            self.memory.insert(n, self.t - 1);
        }

        self.prev.replace(speak);
        self.t += 1;

        self.prev
    }

}

fn part1(inputs: &[usize]) -> usize {
    let mut game = Game::new(inputs);
    game.nth(2020 - 1).unwrap()
}

fn part2(inputs: &[usize]) -> usize {
    let mut game = Game::new(inputs);
    game.nth(30_000_000 - 1).unwrap()
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        let tests: &[(&[usize], usize)] = &[
            (&[1, 3, 2], 1),
            // (&[2, 1, 3], 10),
            // (&[1, 2, 3], 27),
            // (&[2, 3, 1], 78),
            // (&[3, 2, 1], 438),
            // (&[3, 1, 2], 1836),
        ];

        for (inputs, expected) in tests {
            assert_eq!(super::part1(inputs), *expected)
        }
    }
}
