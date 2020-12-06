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
    pub struct Memory(Vec<i64>);

    impl Memory {
        pub fn new(input: &[i64]) -> Memory {
            Memory(input.to_vec())
        }

        pub fn run(&mut self) {
            self.run_on(None);
        }

        pub fn run_on(&mut self, input: Option<i64>) -> Vec<i64> {
            let mut ptr = 0;
            let mut output = Vec::new();
            loop {
                if ptr >= self.0.len() {
                    panic!("overran memory")
                }
                match self.get(ptr as i64) {
                    99 => break,
                    n => {
                        let (op, modes) = parse_op(n.try_into().unwrap());
                        match op {
                            Op::Add => {
                                let x = self.read_param(ptr + 1, modes[0]);
                                let y = self.read_param(ptr + 2, modes[1]);
                                let v = x + y;
                                self.write_result(ptr + 3, modes[2], v)
                            }
                            Op::Mul => {
                                let x = self.read_param(ptr + 1, modes[0]);
                                let y = self.read_param(ptr + 2, modes[1]);
                                let v = x * y;
                                self.write_result(ptr + 3, modes[2], v)
                            }
                            Op::Save => {
                                let input = input.expect("no input provided");
                                self.write_result(ptr + 1, modes[0], input)
                            }
                            Op::Return => {
                                let ret = self.read_param(ptr + 1, modes[0]);
                                output.push(ret);
                            }
                        };
                        ptr += op.num_params() + 1;
                    }
                };
            }
            output
        }

        fn read_param(&self, index: usize, mode: ParamMode) -> i64 {
            match mode {
                ParamMode::Position => self.0[self.0[index as usize] as usize],
                ParamMode::Immediate => self.0[index as usize],
            }
        }

        fn write_result(&mut self, index: usize, mode: ParamMode, value: i64) {
            match mode {
                ParamMode::Position => {
                    let position = self.0[index as usize];
                    self.0[position as usize] = value;
                }
                ParamMode::Immediate => panic!("can't write in immediate mode"),
            }
        }

        pub fn get(&self, index: i64) -> i64 {
            self.0[index as usize]
        }

        pub fn set(&mut self, index: i64, value: i64) {
            self.0[index as usize] = value
        }
    }

    enum Op {
        Add,
        Mul,
        Save,
        Return,
    }

    impl Op {
        fn num_params(&self) -> usize {
            match self {
                Op::Add => 3,
                Op::Mul => 3,
                Op::Save => 1,
                Op::Return => 1,
            }
        }
    }

    impl From<usize> for Op {
        fn from(n: usize) -> Self {
            match n {
                1 => Op::Add,
                2 => Op::Mul,
                3 => Op::Save,
                4 => Op::Return,
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

        #[test]
        fn basics() {
            let tests: &[(&[i64], &[i64])] = &[
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
                let (mut mem, expected) = (Memory::new(&input), Memory::new(&expected));
                mem.run();
                assert_eq!(mem, expected);
            }
        }

        #[test]
        fn parameter_modes() {
            let mut mem = Memory::new(&[1002, 4, 3, 4, 33]);
            mem.run();
            assert_eq!(mem, Memory::new(&[1002, 4, 3, 4, 99]));
        }

        #[test]
        fn save_return() {
            let mut mem = Memory::new(&[3, 0, 4, 0, 99]);
            assert_eq!(mem.run_on(Some(1066)), vec![1066]);
        }
    }
}
