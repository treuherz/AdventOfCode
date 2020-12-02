use aoc19::util::{parse, print_answers};

fn main() -> std::io::Result<()> {
    let inputs: Vec<i64> = parse("inputs/1")?;
    print_answers(&inputs, f1, f2);
    Ok(())
}

fn fuel_step(mass: &i64) -> i64 {
    mass / 3 - 2
}

fn fuel_full(mass: &i64) -> i64 {
    let fuel = fuel_step(&mass);
    if fuel < 0 {
        return 0;
    }
    fuel + fuel_full(&fuel)
}

fn f1(inputs: &Vec<i64>) -> i64 {
    inputs.iter().map(fuel_step).sum()
}

fn f2(inputs: &Vec<i64>) -> i64 {
    inputs.iter().map(fuel_full).sum()
}
