use aoc20::util::{parse, print_answers};
use itertools::Itertools;
use std::{collections::HashSet, time::Instant};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/21")?;
    let foods: Vec<Food> = inputs.iter().map(|l| parse_food(&l)).collect();
    print_answers(21, &foods, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

struct Food {
    ingredients: HashSet<String>,
    allergens: HashSet<String>,
}

fn parse_food(s: &str) -> Food {
    let (ingreds_str, rem) = s.split_once(" (contains ").unwrap();
    let ingredients = ingreds_str.split(' ').map(|s| s.into()).collect();

    let allergens = rem
        .trim_end_matches(')')
        .split(", ")
        .map(|s| s.into())
        .collect();

    Food {
        ingredients,
        allergens,
    }
}

fn part1(foods: &[Food]) -> usize {
    let allergens = foods
        .iter()
        .map(|f| f.allergens.clone())
        .reduce(|ref a, ref b| a | b)
        .unwrap();

    let ingredients = foods
        .iter()
        .map(|f| f.ingredients.clone())
        .reduce(|ref a, ref b| a | b)
        .unwrap();

    let allergen_ingreds = allergens
        .iter()
        .map(|a| {
            foods
                .iter()
                .filter(|f| f.allergens.contains(a))
                .map(|f| f.ingredients.clone())
                .reduce(|ref a, ref b| a & b)
                .unwrap()
        })
        .reduce(|ref a, ref b| a | b)
        .unwrap();

    let safe_ingreds = ingredients.difference(&allergen_ingreds);

    safe_ingreds
        .map(|i| foods.iter().filter(|&f| f.ingredients.contains(i)).count())
        .sum()
}

fn part2(foods: &[Food]) -> String {
    let allergens = foods
        .iter()
        .map(|f| f.allergens.clone())
        .reduce(|ref a, ref b| a | b)
        .unwrap();

    let mut allergen_ingreds: Vec<(&str, HashSet<String>)> = allergens
        .iter()
        .map(|a| {
            let ingreds = foods
                .iter()
                .filter(|f| f.allergens.contains(a))
                .map(|f| f.ingredients.clone())
                .reduce(|ref a, ref b| a & b)
                .unwrap();
            (a.as_str(), ingreds)
        })
        .collect();

    allergen_ingreds.sort_unstable_by_key(|(_, set)| set.len());

    dbg!(&allergen_ingreds);

    let mut assigned: HashSet<String> = HashSet::new();
    while allergen_ingreds.iter().any(|(_, set)| set.len() > 1) {
        allergen_ingreds
            .iter_mut()
            .filter(|(_, set)| set.len() != 1)
            .for_each(|(_, set)| *set = set.difference(&assigned).cloned().collect());
        allergen_ingreds
            .iter()
            .filter(|(_, set)| set.len() == 1)
            .for_each(|(_, set)| assigned = &assigned | set);
    }

    dbg!(&allergen_ingreds);

    allergen_ingreds.sort_unstable_by_key(|&(a, _)| a);

    allergen_ingreds
        .iter()
        .map(|(_, set)| set.iter().next().unwrap())
        .join(",")
}
