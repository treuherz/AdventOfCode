use regex::Regex;

use aoc20::util::{parse, print_answers};

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = parse("inputs/2")?;
    print_answers(2, &inputs, f1, f2);
    Ok(())
}


fn f1(inputs: &Vec<String>) -> usize {
    let re: Regex = Regex::new(r"^(?P<n1>\d+)-(?P<n2>\d+) (?P<letter>[a-z]): (?P<pass>[a-z]+)$").unwrap();

    inputs.iter().filter(|&line| {
        let captures = re.captures(line).unwrap();

        let min: usize = captures.name("n1").unwrap().as_str().parse().unwrap();
        let max: usize = captures.name("n2").unwrap().as_str().parse().unwrap();
        let letter: char = captures.name("letter").unwrap().as_str().chars().next().unwrap();
        let pass: &str = captures.name("pass").unwrap().as_str();

        let count = pass.chars().filter(|c| c == &letter).count();

        min <= count && count <= max
    }).count()
}

fn f2(inputs: &Vec<String>) -> usize {
    let re: Regex = Regex::new(r"^(?P<n1>\d+)-(?P<n2>\d+) (?P<letter>[a-z]): (?P<pass>[a-z]+)$").unwrap();

    inputs.iter().filter(|&line| {
        let captures = re.captures(line).unwrap();

        let n1: usize = captures.name("n1").unwrap().as_str().parse().unwrap();
        let n2: usize = captures.name("n2").unwrap().as_str().parse().unwrap();
        let letter: char = captures.name("letter").unwrap().as_str().chars().next().unwrap();
        let pass: &str = captures.name("pass").unwrap().as_str();

        let at_n1 = pass.chars().nth(n1 - 1).unwrap() == letter;
        let at_n2 = pass.chars().nth(n2 - 1).unwrap() == letter;

        at_n1 != at_n2
    }).count()
}
