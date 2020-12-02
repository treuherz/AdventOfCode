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
