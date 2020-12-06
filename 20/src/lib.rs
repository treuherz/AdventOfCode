pub mod util {
    use std::str::FromStr;
    use std::borrow::Borrow;

    pub fn parse<T: FromStr>(path: &str) -> std::io::Result<Vec<T>> {
        let contents = std::fs::read_to_string(path)?;
        let line_iter = contents.lines().filter_map(|l| l.parse::<T>().ok());
        Ok(line_iter.collect())
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
