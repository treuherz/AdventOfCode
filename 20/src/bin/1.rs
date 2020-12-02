use aoc20::util::{parse, print_answers};

fn main() -> std::io::Result<()> {
    let inputs: Vec<i64> = parse("inputs/1")?;
    print_answers(&inputs, f1, f2);
    Ok(())
}

const TARGET: i64 = 2020;

fn f1(inputs: &Vec<i64>) -> i64 {
    let mut sorted = inputs.clone();
    sorted.sort();

    for (i, n) in sorted.iter().enumerate() {
        let res = sorted[i..].binary_search(&(TARGET - n));
        if let Ok(_) = res {
            return n * (TARGET - n);
        }
    }
    panic!("answer not found");
}

fn f2(inputs: &Vec<i64>) -> i64 {
    let mut sorted = inputs.clone();
    sorted.sort();

    for (i, n) in sorted.iter().enumerate() {
        for (j, m) in sorted[i..].iter().enumerate() {
            let res = sorted[i..][j..].binary_search(&(TARGET - n - m));
            if let Ok(k) = res {
                dbg!(n, m, TARGET - n - m);
                return n * m * (TARGET - n - m);
            }
        }
    }
    panic!("answer not found");
}
