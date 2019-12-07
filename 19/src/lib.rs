pub mod util {
    use std::str::FromStr;

    pub fn parse<T: FromStr>(path: &str) -> std::io::Result<Vec<T>> {
        let contents = std::fs::read_to_string(path)?;
        let line_iter = contents.lines().filter_map(|l| l.parse::<T>().ok());
        Ok(line_iter.collect())
    }

    pub fn print_answers<I, O1, O2, F1, F2>(inputs: &Vec<I>, f1: F1, f2: F2)
    where
        O1: std::fmt::Display,
        O2: std::fmt::Display,
        F1: Fn(&Vec<I>) -> O1,
        F2: Fn(&Vec<I>) -> O2,
    {
        println!("{}", f1(&inputs));
        println!("{}", f2(&inputs));
    }

    pub fn print_answer<I, O, F>(inputs: &Vec<I>, f: F)
    where
        O: std::fmt::Display,
        F: Fn(&Vec<I>) -> O,
    {
        println!("{}", f(&inputs));
    }
}

pub mod computer {
    pub fn run(mut mem: Vec<usize>) -> Vec<usize> {
        let mut cur = 0;
        loop {
            match mem[cur] {
                99 => break,
                n @ _ => match Operation::from_code(&n) {
                    None => panic!(n),
                    Some(op) => {
                        let (idx, val) = op.run(&cur, &mem);
                        mem[idx] = val;
                        cur += op.params() + 1;
                    }
                },
            }
        }
        return mem;
    }

    enum Operation {
        Add,
        Mul,
    }

    impl Operation {
        fn from_code(code: &usize) -> Option<Operation> {
            use Operation::*;
            match code {
                1 => Some(Add),
                2 => Some(Mul),
                _ => None,
            }
        }

        fn params(&self) -> usize {
            use Operation::*;
            match self {
                Add => 3,
                Mul => 3,
            }
        }

        fn apply(&self, params: Vec<usize>, mem: &Vec<usize>) -> (usize, usize) {
            match self {
                Operation::Add => {
                    let (in1, in2, out) = (params[0], params[1], params[2]);
                    (out, mem[in1] + mem[in2])
                }
                Operation::Mul => {
                    let (in1, in2, out) = (params[0], params[1], params[2]);
                    (out, mem[in1] * mem[in2])
                }
            }
        }

        fn read_params(&self, cur: &usize, mem: &Vec<usize>) -> Vec<usize> {
            let count = self.params();
            let mut v = Vec::with_capacity(count);
            for i in 1..=count {
                v.push(mem[cur + i]);
            }
            v
        }

        fn run(&self, cur: &usize, mem: &Vec<usize>) -> (usize, usize) {
            let params = self.read_params(cur, mem);
            self.apply(params, mem)
        }
    }
}
