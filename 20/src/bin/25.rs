use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let inputs: Vec<u64> = parse("inputs/25")?;
    print_answers(25, &inputs, part1, part2);
    Ok(())
}

const MODULUS: u64 = 20201227;

fn transform(subject: u64, loop_size: u64) -> u64 {
    let mut n = 1;
    for _ in 0..loop_size {
        n = (n * subject) % MODULUS;
    }
    n
}

fn find_loop_size(subject: u64, target: u64) -> u64 {
    let mut n = 1;
    let mut count = 1;
    loop {
        n = (n * subject) % MODULUS;
        if n == target {
            break count;
        }
        count += 1;
    }
}

fn part1(inputs: &[u64]) -> u64 {
    let a_pub = inputs[0];
    let b_pub = inputs[1];

    let a_loop = find_loop_size(7, a_pub);
    dbg!(a_loop);
    let b_loop = find_loop_size(7, b_pub);
    dbg!(b_loop);

    let a_shared = transform(a_pub, b_loop);
    let b_shared = transform(b_pub, a_loop);
    assert_eq!(a_shared, b_shared);

    a_shared
}

fn part2(inputs: &[u64]) -> u64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1() {
        assert_eq!(super::part1(&[5764801, 17807724]), 14897079)
    }

    #[test]
    fn part2() {}
}
