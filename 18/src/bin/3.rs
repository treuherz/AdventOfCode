use aoc18::util::{parse, print_ans};
use regex::Regex;
use indicatif::ProgressBar;

fn main() -> std::io::Result<()> {
    let inputs: Vec<String> = parse("inputs/3")?;
    let claims = parse_claims(&inputs);
    let sheet = claim_sheet(&claims, 1000);
    let a1 = f1(&sheet);
    let a2 = f2(&claims, &sheet);
    println!("{}\n{}", a1, a2);
    Ok(())
}

fn parse_claims(inputs: &Vec<String>) -> Vec<Claim> {
    let bar = ProgressBar::new(inputs.len() as u64);
    let claims: Vec<Claim> = inputs.iter().map(|s| {
        bar.inc(1);
        Claim::from(s)
    }).collect();
    bar.finish_and_clear();
    claims
}

fn claim_sheet(claims: &Vec<Claim>, size: usize) -> SqMatrix {
    let mut sheet = SqMatrix::new(0, size);
    for claim in claims {
        for i in claim.y..(claim.y + claim.height) as usize {
            for j in claim.x..(claim.x + claim.width) as usize {
                *sheet.get_mut(i, j).unwrap() += 1;
            }
        }
    }
    sheet
}

fn f1(sheet: &SqMatrix) -> usize {
    sheet.mat.iter().flatten().filter(|i| i >= &&2usize).count()
}

fn f2(claims: &Vec<Claim>, sheet: &SqMatrix) -> usize {
    'main: for claim in claims {
        for i in claim.y..(claim.y + claim.height) as usize {
            for j in claim.x..(claim.x + claim.width) as usize {
                if sheet.get(i, j).unwrap() >= &2 {
                    continue 'main
                }
            }
        }
        return claim.id
    }
    0
}

struct SqMatrix {
    mat: Vec<Vec<usize>>,
    size: usize,
}

impl SqMatrix {
    fn new(fill: usize, size: usize) -> SqMatrix {
        SqMatrix {
            mat: vec![vec![fill; size]; size],
            size,
        }
    }

    fn get(&self, row: usize, col: usize) -> Option<&usize> {
        self.mat.get(row)?.get(col)
    }

    fn get_mut(&mut self, row: usize, col: usize) -> Option<&mut usize> {
        self.mat.get_mut(row)?.get_mut(col)
    }
}

struct Claim {
    id: usize,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

impl From<&String> for Claim {
    fn from(string: &String) -> Claim {
        let re = Regex::new(r"^#(\d+) @ (\d+),(\d+): (\d+)x(\d+)$").unwrap();
        let caps = re.captures(string).unwrap();
        let id = caps.get(1).unwrap().as_str().parse().unwrap();
        let x = caps.get(2).unwrap().as_str().parse().unwrap();
        let y = caps.get(3).unwrap().as_str().parse().unwrap();
        let width = caps.get(4).unwrap().as_str().parse().unwrap();
        let height = caps.get(5).unwrap().as_str().parse().unwrap();
        Claim {
            id,
            x,
            y,
            width,
            height,
        }
    }
}
