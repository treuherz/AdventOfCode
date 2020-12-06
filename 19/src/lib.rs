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

pub mod computer {
    use crate::computer::ParamMode::*;
    use std::borrow::BorrowMut;
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;
    use Operation::*;

    pub fn run(mut mem: Memory) -> Memory {
        let mut cur = 0;
        loop {
            match mem.get(cur) {
                99 => break,
                n @ _ => {
                    let op = Operation::from_code(&n);
                    op.run(&cur, &mut mem);
                    cur += op.params() + 1;
                }
            }
        }
        return mem;
    }

    #[derive(Debug, Clone)]
    pub struct Memory(Vec<Cell<usize>>);

    impl Memory {
        pub fn new(data: &Vec<usize>) -> Memory {
            Memory(data.iter().map(|n| Cell::new(*n)).collect())
        }

        pub fn get(&self, pos: usize) -> usize {
            self.0[pos].get()
        }

        pub fn set(&mut self, pos: usize, val: usize) {
            self.0[pos] = Cell::new(val);
        }

        //        fn read_param<'a>(&'a mut self, pos: usize, mode: ParamMode) -> &'a mut usize {
        //            match mode {
        //                Position => self.0.get_mut(pos).map(|c| c.get_mut()).unwrap(),
        //                Immediate => self.0.get(pos).map(|c| &mut c.get()).unwrap(),
        //            }
        //        }
    }

    #[derive(Clone)]
    enum ParamMode {
        Position,
        Immediate,
    }

    enum Operation {
        Add(Vec<ParamMode>),
        Mul(Vec<ParamMode>),
    }

    impl Operation {
        fn from_code(code: &usize) -> Operation {
            let mut mode_code = *code / 100;
            let mut modes = Vec::new();
            let mode_code_iter = std::iter::from_fn(|| {
                let next = mode_code % 10;
                mode_code /= 10;
                Some(next)
            })
            .take(Self::params_from_code(&(code % 100)));
            for d in mode_code_iter {
                match d {
                    0 => modes.push(Position),
                    1 => modes.push(Immediate),
                    _ => panic!(),
                }
            }
            match code % 100 {
                1 => Add(modes),
                2 => Mul(modes),
                _ => panic!(),
            }
        }

        fn params_from_code(code: &usize) -> usize {
            match code {
                1 => 3,
                2 => 3,
                _ => panic!(),
            }
        }

        fn params(&self) -> usize {
            match self {
                Add(_) => 3,
                Mul(_) => 3,
                _ => panic!(),
            }
        }

        fn apply(&self, mut params: Vec<&mut usize>, mem: &Memory) {
            match self {
                Add(_) => {
                    let mut out = params[2];
                    out = *params[0] + *params[1];
                }
                Mul(_) => {
                    let mut out = params[2];
                    out = *params[0] * *params[1];
                }
            }
        }

        fn read_params<'a>(&self, mem: &'a mut Memory, cur: &usize) -> Vec<&'a mut usize> {
            match self {
                Add(m) => Self::read_params_by_mode(mem, cur, m),
                Mul(m) => Self::read_params_by_mode(mem, cur, m),
            }
        }

        fn read_params_by_mode<'a>(
            mem: &'a mut Memory,
            cur: &usize,
            modes: &Vec<ParamMode>,
        ) -> Vec<&'a mut usize> {
            let mut params = Vec::with_capacity(modes.len());
            for (i, mode) in modes.iter().enumerate() {
                let pos = cur + i + 1;
                let mut owned: usize;
                let p = match mode {
                    Position => mem.0.get_mut(pos).unwrap().get_mut(),
                    Immediate => {
                        owned = mem.0.get(pos).unwrap().get();
                        &mut owned
                    }
                };
                params.push(p);
            }
            params
        }

        fn run(&self, cur: &usize, mem: &mut Memory) {
            let params = self.read_params(mem, cur);
            self.apply(params, mem);
        }
    }
}
