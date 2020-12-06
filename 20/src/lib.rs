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
