use aoc20::util::{parse, print_answers};

fn main() -> anyhow::Result<()> {
    let inputs: Vec<String> = parse("inputs/3")?;
    let map = Map::from_inputs(&inputs);
    print_answers(&map, f1, f2);
    Ok(())
}

const WIDTH: usize = 31;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Space {
    Empty,
    Tree,
}

type Line = [Space; WIDTH];
type Coord = [usize; 2];

struct Map {
    data: Vec<Line>,
}

impl Map {
    fn from_inputs(inputs: &Vec<String>) -> Map {
        let mut data: Vec<Line> = vec![];
        for s in inputs {
            let mut line = [Space::Empty; WIDTH];
            for (x, c) in s.chars().enumerate() {
                if c == '#' {
                    line[x] = Space::Tree
                }
            }
            data.push(line)
        }
        Map { data }
    }

    fn height(&self) -> usize {
        self.data.len()
    }

    fn tree_at(&self, idx: Coord) -> bool {
        self.data[idx[0]][idx[1] % WIDTH] == Space::Tree
    }
}

fn f1(map: &Map) -> i64 {
    let mut acc: i64 = 0;
    let (mut x, mut y): (usize, usize) = (0, 0);
    while y < map.height() {
        if map.tree_at([y, x]) {
            acc += 1
        }
        x += 3;
        y += 1
    }
    acc
}

fn del(y: usize, x: usize) -> impl Fn(Coord) -> Coord {
    move |c| [c[0] + y, c[1] + x]
}

fn f2(map: &Map) -> i64 {
    let mut acc: i64 = 1;
    let slopes = vec![del(1, 1), del(1, 3), del(1, 5), del(1, 7), del(2, 1)];
    for f in slopes {
        let mut trees: i64 = 0;
        let mut c: Coord = [0, 0];
        while c[0] < map.height() {
            if map.tree_at(c) {
                trees += 1
            }
            c = f(c)
        }
        acc *= trees
    }
    acc
}
