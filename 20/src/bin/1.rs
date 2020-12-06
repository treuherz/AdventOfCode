use aoc20::util::{parse, print_answers};

fn main() -> std::io::Result<()> {
    let inputs: Vec<i64> = parse("inputs/1")?;
    print_answers(1, &inputs, f1, f2);
    Ok(())
}

const TARGET: i64 = 2020;

fn f1(inputs: &[i64]) -> i64 {
    let mut sorted = inputs.to_owned();
    sorted.sort_unstable();

    for (i, n) in sorted.iter().enumerate() {
        let res = sorted[i..].binary_search(&(TARGET - n));
        if res.is_ok() {
            return n * (TARGET - n);
        }
    }
    panic!("answer not found");
}

fn f2(inputs: &[i64]) -> i64 {
    let mut sorted = inputs.to_owned();
    sorted.sort_unstable();

    for (i, n) in sorted.iter().enumerate() {
        for (j, m) in sorted[i..].iter().enumerate() {
            let res = sorted[i..][j..].binary_search(&(TARGET - n - m));
            if res.is_ok() {
                return n * m * (TARGET - n - m);
            }
        }
    }
    panic!("answer not found");
}
