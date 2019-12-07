pub mod util {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use std::str::FromStr;

    pub fn parse<T: FromStr>(path: &str) -> std::io::Result<Vec<T>> {
        let buf_reader = BufReader::new(File::open(path)?);
        let line_iter = buf_reader
            .lines()
            .filter_map(|l| l.unwrap().parse::<T>().ok());
        Ok(line_iter.collect())
    }

    pub fn print_ans<I, O1, O2, F1, F2>(inputs: &Vec<I>, f1: F1, f2: F2)
    where
        O1: std::fmt::Display,
        O2: std::fmt::Display,
        F1: Fn(&Vec<I>) -> O1,
        F2: Fn(&Vec<I>) -> O2,
    {
        println!("{}", f1(&inputs));
        println!("{}", f2(&inputs));
    }
}
