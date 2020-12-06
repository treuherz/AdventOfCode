#![feature(default_free_fn)]

pub mod util {
    use anyhow::anyhow;
    use std::borrow::Borrow;
    use std::str::FromStr;

    pub fn parse<T>(path: &str) -> anyhow::Result<Vec<T>>
    where
        T: FromStr,
        <T as FromStr>::Err: std::error::Error + Send + Sync + 'static,
    {
        let contents = std::fs::read_to_string(path)?;
        contents
            .lines()
            .map(|l| l.parse::<T>().map_err(|e| anyhow!(e)))
            .collect()
    }

    pub fn print_answers<I, J, O1, O2, F1, F2>(day: u32, input: &I, part1: F1, part2: F2)
    where
        O1: std::fmt::Display,
        O2: std::fmt::Display,
        I: Borrow<J>,
        J: ?Sized,
        F1: Fn(&J) -> O1,
        F2: Fn(&J) -> O2,
    {
        println!("─── Day {}, Part 1 ───", day);
        println!("{}", part1(input.borrow()));
        println!();
        println!("─── Day {}, Part 2 ───", day);
        println!("{}", part2(input.borrow()));
    }
}

pub mod intcode {
    use std::convert::TryInto;

    pub fn parse_memory(path: &str) -> anyhow::Result<Vec<i64>> {
        let s = std::fs::read_to_string(path)?;
        let mut out = Vec::new();
        for i in s.trim_end().split(',') {
            out.push(i.parse()?);
        }
        Ok(out)
    }

    #[derive(Debug, Eq, PartialEq)]
    pub struct Computer {
        memory: Vec<i64>,
        pointer: usize,
    }

    impl Computer {
        pub fn new(input: &[i64]) -> Computer {
            Computer {
                memory: input.to_vec(),
                pointer: 0,
            }
        }

        pub fn run(&mut self) {
            self.run_on(None);
        }

        pub fn run_on(&mut self, input: Option<i64>) -> Vec<i64> {
            let mut output = Vec::new();
            let mut should_advance = true;
            loop {
                if self.pointer >= self.memory.len() {
                    panic!("overran memory")
                }
                match self.get(self.pointer as i64) {
                    99 => break,
                    n => {
                        let (op, modes) = parse_op(n.try_into().unwrap());
                        match op {
                            Op::Add => {
                                let x = self.read_param(1, modes[0]);
                                let y = self.read_param(2, modes[1]);
                                let v = x + y;
                                self.write_result(3, modes[2], v)
                            }
                            Op::Multiply => {
                                let x = self.read_param(1, modes[0]);
                                let y = self.read_param(2, modes[1]);
                                let v = x * y;
                                self.write_result(3, modes[2], v)
                            }
                            Op::Save => {
                                let input = input.expect("no input provided");
                                self.write_result(1, modes[0], input)
                            }
                            Op::Return => {
                                let ret = self.read_param(1, modes[0]);
                                output.push(ret);
                            }
                            Op::JumpIfTrue => {
                                if self.read_param(1, modes[0]) != 0 {
                                    let new = self.read_param(2, modes[1]) as usize;
                                    should_advance = false;
                                    self.pointer = new;
                                }
                            }
                            Op::JumpIfFalse => {
                                if self.read_param(1, modes[0]) == 0 {
                                    let new = self.read_param(2, modes[1]) as usize;
                                    should_advance = false;
                                    self.pointer = new;
                                }
                            }
                            Op::LessThan => {
                                let x = self.read_param(1, modes[0]);
                                let y = self.read_param(2, modes[1]);
                                self.write_result(3, modes[2], (x < y).into());
                            }
                            Op::Equals => {
                                let x = self.read_param(1, modes[0]);
                                let y = self.read_param(2, modes[1]);
                                self.write_result(3, modes[2], (x == y).into());
                            }
                        };
                        if should_advance {
                            self.pointer += op.num_params() + 1;
                        } else {
                            should_advance = true;
                        }
                    }
                };
            }
            output
        }

        fn read_param(&self, offset: usize, mode: ParamMode) -> i64 {
            match mode {
                ParamMode::Position => {
                    self.memory[self.memory[(self.pointer + offset) as usize] as usize]
                }
                ParamMode::Immediate => self.memory[(self.pointer + offset) as usize],
            }
        }

        fn write_result(&mut self, offset: usize, mode: ParamMode, value: i64) {
            match mode {
                ParamMode::Position => {
                    let position = self.memory[(self.pointer + offset) as usize];
                    self.memory[position as usize] = value;
                }
                ParamMode::Immediate => panic!("can't write in immediate mode"),
            }
        }

        pub fn get(&self, index: i64) -> i64 {
            self.memory[index as usize]
        }

        pub fn set(&mut self, index: i64, value: i64) {
            self.memory[index as usize] = value
        }
    }

    enum Op {
        Add,
        Multiply,
        Save,
        Return,
        JumpIfTrue,
        JumpIfFalse,
        LessThan,
        Equals,
    }

    impl Op {
        fn num_params(&self) -> usize {
            match self {
                Op::Add => 3,
                Op::Multiply => 3,
                Op::Save => 1,
                Op::Return => 1,
                Op::JumpIfTrue => 2,
                Op::JumpIfFalse => 2,
                Op::LessThan => 3,
                Op::Equals => 3,
            }
        }
    }

    impl From<usize> for Op {
        fn from(n: usize) -> Self {
            match n {
                1 => Op::Add,
                2 => Op::Multiply,
                3 => Op::Save,
                4 => Op::Return,
                5 => Op::JumpIfTrue,
                6 => Op::JumpIfFalse,
                7 => Op::LessThan,
                8 => Op::Equals,
                _ => panic!("unrecognised opcode"),
            }
        }
    }

    #[derive(Clone, Copy)]
    enum ParamMode {
        Position,
        Immediate,
    }

    impl From<usize> for ParamMode {
        fn from(n: usize) -> Self {
            match n {
                0 => ParamMode::Position,
                1 => ParamMode::Immediate,
                _ => panic!("unrecognised parameter mode"),
            }
        }
    }

    impl Default for ParamMode {
        fn default() -> Self {
            ParamMode::Position
        }
    }

    fn parse_op(code: usize) -> (Op, Vec<ParamMode>) {
        let op = Op::from(code % 100);

        let mut mode_code = code / 100;
        let modes = std::iter::from_fn(|| {
            let next = mode_code % 10;
            mode_code /= 10;
            Some(next)
        })
        .take(op.num_params())
        .map(|d| ParamMode::from(d))
        .collect();

        (op, modes)
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use std::cmp::Ordering;

        #[test]
        fn basics() {
            let tests: Vec<(&[i64], &[i64])> = vec![
                (&[1, 0, 0, 0, 99], &[2, 0, 0, 0, 99]),
                (&[2, 3, 0, 3, 99], &[2, 3, 0, 6, 99]),
                (&[2, 4, 4, 5, 99, 0], &[2, 4, 4, 5, 99, 9801]),
                (
                    &[1, 1, 1, 4, 99, 5, 6, 0, 99],
                    &[30, 1, 1, 4, 2, 5, 6, 0, 99],
                ),
                (
                    &[1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50],
                    &[3500, 9, 10, 70, 2, 3, 11, 0, 99, 30, 40, 50],
                ),
            ];
            for (input, expected) in tests {
                let (mut mem, expected) = (Computer::new(&input), Computer::new(&expected));
                mem.run();
                assert_eq!(mem, expected);
            }
        }

        #[test]
        fn parameter_modes() {
            let mut mem = Computer::new(&[1002, 4, 3, 4, 33]);
            mem.run();
            assert_eq!(mem, Computer::new(&[1002, 4, 3, 4, 99]));
        }

        #[test]
        fn save_return() {
            let mut mem = Computer::new(&[3, 0, 4, 0, 99]);
            assert_eq!(mem.run_on(Some(1066)), vec![1066]);
        }

        #[test]
        fn comparisons() {
            let tests: Vec<(&[i64], Box<dyn Fn(i64, i64) -> bool>)> = vec![
                (
                    &[3, 9, 8, 9, 10, 9, 4, 9, 99, -1, 8],
                    Box::new(|x, y| x == y),
                ),
                (
                    &[3, 9, 7, 9, 10, 9, 4, 9, 99, -1, 8],
                    Box::new(|x, y| x < y),
                ),
                (&[3, 3, 1108, -1, 8, 3, 4, 3, 99], Box::new(|x, y| x == y)),
                (&[3, 3, 1107, -1, 8, 3, 4, 3, 99], Box::new(|x, y| x < y)),
            ];

            for (input, cmp) in tests {
                for n in 0..16 {
                    let mut computer = Computer::new(input);
                    let output = computer.run_on(Some(n));
                    let expected = vec![i64::from(cmp(n, 8))];
                    assert_eq!(
                        output, expected,
                        "memory = {:?}, input = {}, got {:?}, want {:?}",
                        input, n, output, expected
                    );
                }
            }
        }

        #[test]
        fn jumps() {
            let tests: Vec<&[i64]> = vec![
                &[3, 12, 6, 12, 15, 1, 13, 14, 13, 4, 13, 99, -1, 0, 1, 9],
                &[3, 3, 1105, -1, 9, 1101, 0, 0, 12, 4, 12, 99, 1],
            ];

            for input in tests {
                for n in 0..5 {
                    let mut computer = Computer::new(input);
                    let output = computer.run_on(Some(n));
                    let expected = vec![i64::from(n != 0)];
                    assert_eq!(
                        output, expected,
                        "memory = {:?}, input = {}, got {:?}, want {:?}",
                        input, n, output, expected
                    );
                }
            }
        }

        #[test]
        fn jumps_galore() {
            let input = &[
                3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36,
                98, 0, 0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000,
                1, 20, 4, 20, 1105, 1, 46, 98, 99,
            ];

            for n in 6..11 {
                let mut computer = Computer::new(input);
                let output = computer.run_on(Some(n));
                let expected = vec![match n.cmp(&8) {
                    Ordering::Less => 999,
                    Ordering::Equal => 1000,
                    Ordering::Greater => 1001,
                }];
                assert_eq!(
                    output, expected,
                    "input = {}, got {:?}, want {:?}",
                    n, output, expected
                );
            }
        }
    }
}
