use aoc20::util::{parse, print_answers};
use std::{
    cmp::Ordering,
    collections::VecDeque,
    convert::TryInto,
    hash::{Hash, Hasher},
    collections::HashSet,
    time::Instant,
};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/22")?;
    let decks = parse_decks(&inputs);
    print_answers(22, &decks, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

type Deck = VecDeque<usize>;

fn parse_decks(input: &[String]) -> [Deck; 2] {
    input
        .splitn(2, |s| s.is_empty())
        .map(|ss| {
            ss[1..]
                .iter()
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<_>>()
        .try_into()
        .unwrap()
}

fn part1(decks: &[Deck; 2]) -> usize {
    let mut player1 = decks[0].clone();
    let mut player2 = decks[1].clone();

    while !player1.is_empty() && !player2.is_empty() {
        let (winner, loser) = match player1[0].cmp(&player2[0]) {
            Ordering::Greater => (&mut player1, &mut player2),
            Ordering::Less => (&mut player2, &mut player1),
            Ordering::Equal => unreachable!(),
        };
        winner.rotate_left(1);
        winner.push_back(loser.pop_front().unwrap())
    }

    player1
        .iter()
        .chain(player2.iter())
        .rev()
        .enumerate()
        .map(|(i, n)| (i + 1) * n)
        .sum()
}

fn play(decks: [Deck; 2]) -> ([usize; 2], usize) {
    let [mut player1, mut player2] = decks;

    let mut seen: HashSet<u64> = HashSet::new();

    let mut repeat = false;

    while !player1.is_empty() && !player2.is_empty() {
        if seen.contains(&hash(&[&player1, &player2])) {
            repeat = true;
            break;
        }
        seen.insert(hash(&[&player1, &player2]));

        let card1 = player1[0];
        let card2 = player2[0];

        let (winner, loser) = if player1.len() > card1 && player2.len() > card2 {
            let (_, sub_winner) = play([
                player1.iter().skip(1).take(card1).copied().collect(),
                player2.iter().skip(1).take(card2).copied().collect(),
            ]);
            match sub_winner {
                1 => (&mut player1, &mut player2),
                2 => (&mut player2, &mut player1),
                _ => unreachable!(),
            }
        } else {
            match card1.cmp(&card2) {
                Ordering::Greater => (&mut player1, &mut player2),
                Ordering::Less => (&mut player2, &mut player1),
                Ordering::Equal => unreachable!(),
            }
        };

        winner.rotate_left(1);
        winner.push_back(loser.pop_front().unwrap());
    }

    let winner = if repeat {
        1
    } else {
        match (player1.len(), player2.len()) {
            (_, 0) => 1,
            (0, _) => 2,
            _ => unreachable!(),
        }
    };

    (
        [
            player1
                .iter()
                .rev()
                .enumerate()
                .map(|(i, n)| (i + 1) * n)
                .sum(),
            player2
                .iter()
                .rev()
                .enumerate()
                .map(|(i, n)| (i + 1) * n)
                .sum(),
        ],
        winner,
    )
}

fn part2(decks: &[Deck; 2]) -> usize {
    let (scores, _) = play(decks.clone());
    scores.iter().sum()
}

fn hash<H: Hash>(arrangement: &H) -> u64 {
    let h = &mut std::collections::hash_map::DefaultHasher::new();
    arrangement.hash(h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part2() {
        let decks: [Deck; 2] = [
            VecDeque::from(vec![9, 2, 6, 3, 1]),
            VecDeque::from(vec![5, 8, 4, 7, 10]),
        ];

        let (scores, winner) = play(decks);

        assert_eq!(winner, 2);
        assert_eq!(scores, [0, 291]);
    }
}
