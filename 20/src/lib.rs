pub mod util {
    use std::str::FromStr;

    pub fn parse<T: FromStr>(path: &str) -> std::io::Result<Vec<T>> {
        let contents = std::fs::read_to_string(path)?;
        let line_iter = contents.lines().filter_map(|l| l.parse::<T>().ok());
        Ok(line_iter.collect())
    }

    pub fn print_answers<I, O1, O2, F1, F2>(input: &I, f1: F1, f2: F2)
    where
        O1: std::fmt::Display,
        O2: std::fmt::Display,
        F1: Fn(&I) -> O1,
        F2: Fn(&I) -> O2,
    {
        println!("{}", f1(&input));
        println!("{}", f2(&input));
    }

    pub fn print_answer<I, O, F>(input: &I, f: F)
    where
        O: std::fmt::Display,
        F: Fn(&I) -> O,
    {
        println!("{}", f(&input));
    }
}
