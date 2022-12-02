use aoc20::util::{parse, print_answers};
use itertools::iproduct;
use ndarray::{s, Array2};
use num::Integer;
use regex::Regex;
use std::{
    cmp::min,
    collections::{HashMap, HashSet},
    hash::{Hash, Hasher},
    time::Instant,
};

fn main() -> anyhow::Result<()> {
    let now = Instant::now();
    let inputs: Vec<String> = parse("inputs/20")?;
    let tiles = parse_tiles(&inputs);
    print_answers(20, &tiles, part1, part2);
    println!("Overall time: {:?}", now.elapsed());
    Ok(())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    fn at(n: u64) -> Side {
        match n % 4 {
            0 => Side::Top,
            1 => Side::Right,
            2 => Side::Bottom,
            3 => Side::Left,
            _ => unreachable!(),
        }
    }
}

type Edge = Vec<bool>;

type TileId = u64;

#[derive(Clone, Hash, Debug)]
struct Tile {
    id: TileId,
    data: Array2<bool>,
}

impl Tile {
    fn new(id: TileId, strings: &[String]) -> Tile {
        let data = Tile::parse_strings(strings);
        Tile { id, data }
    }

    fn parse_strings(strings: &[String]) -> Array2<bool> {
        let mut data = Array2::from_elem((strings.len(), strings.len()), false);
        for (y, line) in strings.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    data[[y, x]] = true;
                }
            }
        }
        data
    }

    fn edge(&self, side: Side) -> Edge {
        match side {
            Side::Top => self.data.row(0).to_vec(),
            Side::Right => self.data.column(self.data.ncols() - 1).to_vec(),
            Side::Bottom => self.data.row(self.data.nrows() - 1).to_vec(),
            Side::Left => self.data.column(0).to_vec(),
        }
    }

    fn transformed_edge(&self, rot: Rotations, flip: bool, side: Side) -> Edge {
        use {Rotations::*, Side::*};

        let rev = |mut e: Edge| -> Edge {
            e.reverse();
            e
        };

        match (side, rot, flip) {
            (Top, Zero, false) => self.edge(Top),
            (Top, Once, false) => rev(self.edge(Left)),
            (Top, Twice, false) => rev(self.edge(Bottom)),
            (Top, Thrice, false) => self.edge(Right),
            (Right, Zero, false) => self.edge(Right),
            (Right, Once, false) => self.edge(Top),
            (Right, Twice, false) => rev(self.edge(Left)),
            (Right, Thrice, false) => rev(self.edge(Bottom)),
            (Bottom, Zero, false) => self.edge(Bottom),
            (Bottom, Once, false) => rev(self.edge(Right)),
            (Bottom, Twice, false) => rev(self.edge(Top)),
            (Bottom, Thrice, false) => self.edge(Left),
            (Left, Zero, false) => self.edge(Left),
            (Left, Once, false) => self.edge(Bottom),
            (Left, Twice, false) => rev(self.edge(Right)),
            (Left, Thrice, false) => rev(self.edge(Top)),
            (Top, Zero, true) => rev(self.edge(Top)),
            (Top, Once, true) => rev(self.edge(Right)),
            (Top, Twice, true) => self.edge(Bottom),
            (Top, Thrice, true) => self.edge(Left),
            (Right, Zero, true) => self.edge(Left),
            (Right, Once, true) => rev(self.edge(Top)),
            (Right, Twice, true) => rev(self.edge(Right)),
            (Right, Thrice, true) => self.edge(Bottom),
            (Bottom, Zero, true) => rev(self.edge(Bottom)),
            (Bottom, Once, true) => rev(self.edge(Left)),
            (Bottom, Twice, true) => self.edge(Top),
            (Bottom, Thrice, true) => self.edge(Right),
            (Left, Zero, true) => self.edge(Right),
            (Left, Once, true) => rev(self.edge(Bottom)),
            (Left, Twice, true) => rev(self.edge(Left)),
            (Left, Thrice, true) => self.edge(Top),
        }
    }
}

#[derive(Copy, Clone, Hash, Debug)]
enum Rotations {
    Zero,
    Once,
    Twice,
    Thrice,
}

#[derive(Hash, Debug)]
struct PlacedTile {
    tile: Tile,
    rot: Rotations,
    flip: bool,
}

impl PlacedTile {
    fn new(tile: Tile) -> Self {
        PlacedTile {
            tile,
            rot: Rotations::Zero,
            flip: false,
        }
    }

    fn edge(&self, side: Side) -> Edge {
        self.tile.transformed_edge(self.rot, self.flip, side)
    }

    fn transformed(&self) -> Array2<bool> {
        let flip = self.flip;
        let rot = self.rot;
        let data = &self.tile.data;
        transform(data, flip, rot)
    }
}

fn transform(data: &Array2<bool>, flip: bool, rot: Rotations) -> Array2<bool> {
    let step = if flip { -1 } else { 1 };
    let flipped = data.slice(s![.., ..;step]);
    let rotated = match rot {
        Rotations::Zero => flipped.to_owned(),
        Rotations::Once => flipped.t().slice(s![.., ..;-1]).to_owned(),
        Rotations::Twice => flipped.slice(s![..;-1, ..;-1]).to_owned(),
        Rotations::Thrice => flipped.t().slice(s![..;-1, ..]).to_owned(),
    };

    rotated
}

fn corners(tiles: &HashMap<TileId, Tile>) -> Vec<u64> {
    let unique_edges = unique_edge_counts(tiles);

    let corners: Vec<TileId> = unique_edges
        .iter()
        .filter(|(_, &n)| n == 2)
        .map(|(&k, _)| k)
        .collect();
    assert_eq!(corners.len(), 4);
    corners
}

fn unique_edge_counts(tiles: &HashMap<TileId, Tile>) -> HashMap<TileId, u8> {
    let edges_tiles = build_edge_lookup(tiles);

    let mut unique_edges: HashMap<TileId, u8> = HashMap::new();
    edges_tiles
        .values()
        .filter(|&s| s.len() == 1)
        .for_each(|s| {
            let id = *s.iter().next().unwrap();
            if let Some(n) = unique_edges.get_mut(&id) {
                *n += 1;
            } else {
                unique_edges.insert(id, 1);
            }
        });
    unique_edges
}

fn build_edge_lookup(tiles: &HashMap<TileId, Tile>) -> HashMap<u64, HashSet<TileId>> {
    let mut edges_tiles: HashMap<u64, HashSet<TileId>> = HashMap::new();
    for block in tiles.values() {
        for side in (0..4).map(Side::at) {
            let edge = block.edge(side);

            let k = key(&edge);

            if let Some(set) = edges_tiles.get_mut(&k) {
                set.insert(block.id);
            } else {
                let mut set: HashSet<TileId> = HashSet::new();
                set.insert(block.id);
                edges_tiles.insert(k, set);
            }
        }
    }
    edges_tiles
}

fn key(edge: &Edge) -> u64 {
    let a = hash(edge);

    let rev = {
        let mut e = edge.to_owned();
        e.reverse();
        e
    };
    let b = hash(&rev);

    min(a, b)
}

fn parse_tiles(inputs: &[String]) -> HashMap<TileId, Tile> {
    let tiles: HashMap<TileId, Tile> = inputs
        .split(|s| s.is_empty())
        .map(|g| {
            let (title, pixels) = g.split_first().unwrap();
            let id: u64 = Regex::new(r"^Tile (\d+):$")
                .unwrap()
                .captures(&title)
                .unwrap()[1]
                .parse()
                .unwrap();

            (id, Tile::new(id, pixels))
        })
        .collect();
    tiles
}

fn transforms() -> impl Iterator<Item = (Rotations, bool)> {
    use Rotations::*;
    return iproduct!(
        [Zero, Once, Twice, Thrice].iter().cloned(),
        [false, true].iter().cloned()
    );
}

fn part1(tiles: &HashMap<TileId, Tile>) -> u64 {
    corners(tiles).iter().product()
}

fn part2(tiles: &HashMap<TileId, Tile>) -> usize {
    use Side::*;

    let edge_lookup = build_edge_lookup(tiles);
    let is_unique = |edge: Edge| -> bool { edge_lookup.get(&key(&edge)).unwrap().len() == 1 };

    let mut tiles = tiles.clone();
    let mut nw = PlacedTile::new(tiles.remove(&2593).unwrap());
    let is_nw_oriented =
        |tile: &PlacedTile| -> bool { is_unique(tile.edge(Top)) && is_unique(tile.edge(Left)) };
    for (rot, flip) in transforms() {
        if !is_nw_oriented(&nw) {
            nw = PlacedTile {
                tile: nw.tile,
                rot,
                flip,
            }
        } else {
            break;
        }
    }

    let mut tried: HashMap<u64, HashSet<TileId>> = HashMap::new();

    let mut arrangement: Vec<PlacedTile> = vec![nw];
    'outer: while arrangement.len() < 144 {
        let is_tried = |id: &u64| -> bool {
            tried
                .get(&hash(&arrangement))
                .and_then(|set| set.get(&id))
                .is_some()
        };

        let candidate = |i: usize| -> Option<&TileId> {
            if i < 12 {
                let to_match = arrangement[i - 1].edge(Right);
                edge_lookup
                    .get(&key(&to_match))
                    .map(|set| set.iter())
                    .and_then(|mut it| it.find(|&id| tiles.contains_key(id) && !is_tried(id)))
            } else if i % 12 == 0 {
                let to_match = arrangement[i - 12].edge(Bottom);
                edge_lookup
                    .get(&key(&to_match))
                    .map(|set| set.iter())
                    .and_then(|mut it| it.find(|&id| tiles.contains_key(id) && !is_tried(id)))
            } else {
                let to_match = (
                    arrangement[i - 1].edge(Right),
                    arrangement[i - 12].edge(Bottom),
                );
                let mut intersection = edge_lookup
                    .get(&key(&to_match.0))
                    .and_then(|set1| {
                        edge_lookup
                            .get(&key(&to_match.1))
                            .map(|set2| set1.intersection(set2))
                    })
                    .unwrap();
                intersection.find(|&id| tiles.contains_key(&id) && !is_tried(id))
            }
        };

        let i = arrangement.len();
        let id = candidate(i);
        if id.is_none() {
            let tile = arrangement.pop().unwrap().tile;
            if let Some(set) = tried.get_mut(&hash(&arrangement)) {
                set.insert(tile.id);
            } else {
                let mut set: HashSet<u64> = HashSet::new();
                set.insert(tile.id);
                tried.insert(hash(&arrangement), set);
            }
            tiles.insert(tile.id, tile);
            continue;
        }
        let tile = tiles.remove(id.unwrap()).unwrap();

        for (rot, flip) in transforms() {
            let fits = if i < 12 {
                let right = tile.transformed_edge(rot, flip, Left);
                let left = arrangement[i - 1].edge(Right);
                right == left
            } else if i % 12 == 0 {
                tile.transformed_edge(rot, flip, Top) == arrangement[i - 12].edge(Bottom)
            } else {
                tile.transformed_edge(rot, flip, Left) == arrangement[i - 1].edge(Right)
                    && tile.transformed_edge(rot, flip, Top) == arrangement[i - 12].edge(Bottom)
            };
            if fits {
                arrangement.push(PlacedTile { tile, rot, flip });
                continue 'outer;
            }
        }

        if let Some(set) = tried.get_mut(&hash(&arrangement)) {
            set.insert(tile.id);
        } else {
            let mut set: HashSet<TileId> = HashSet::new();
            set.insert(tile.id);
            tried.insert(hash(&arrangement), set);
        }
    }

    assert_eq!(
        arrangement[0].tile.id
            * arrangement[11].tile.id
            * arrangement[132].tile.id
            * arrangement[143].tile.id,
        66020135789767
    );

    let mut image = Array2::from_elem((8 * 12, 8 * 12), false);

    for (i, tile) in arrangement.iter().enumerate() {
        let (row, col) = i.div_rem(&12usize);
        let mut dst = image.slice_mut(s![row * 8..row * 8 + 8, col * 8..col * 8 + 8]);
        let data = tile.transformed();
        let src = data.slice(s![1..=8, 1..=8]);
        dst.iter_mut()
            .zip(src.iter())
            .enumerate()
            .for_each(|(_, (a, b))| *a = *b);
    }

    for row in image.rows() {
        for &b in row.iter() {
            print!("{}", if b { '#' } else { '.' });
        }
        println!();
    }

    let monster_indices = vec![
        (0, 18),
        (1, 0),
        (1, 5),
        (1, 6),
        (1, 11),
        (1, 12),
        (1, 17),
        (1, 18),
        (1, 19),
        (2, 1),
        (2, 4),
        (2, 7),
        (2, 10),
        (2, 13),
        (2, 16),
    ];
    let mut it = transforms();
    let mut transformed = image.clone();
    let num_monsters = loop {
        let num = transformed
            .windows((3, 20))
            .into_iter()
            .filter(|w| monster_indices.iter().all(|&idx| w[idx]))
            .count();
        if num != 0 {
            break num;
        }
        let (rot, flip) = it.next().unwrap();
        transformed = transform(&image, flip, rot);
    };
    dbg!(num_monsters);

    image.iter().filter(|&&b| b).count() - monster_indices.len() * num_monsters
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
    fn edges() {
        let data: Vec<String> = strings(vec![
            "..##.#..#.",
            "##..#.....",
            "#...##..#.",
            "####.#...#",
            "##.##.###.",
            "##...#.###",
            ".#.#.#..##",
            "..#....#..",
            "###...#.#.",
            "..###..###",
        ]);
        let tile = Tile::new(0, &data);

        use Side::*;
        assert_eq!(
            tile.edge(Top),
            vec![false, false, true, true, false, true, false, false, true, false],
            "Top edge"
        );
        assert_eq!(
            tile.edge(Right),
            vec![false, false, false, true, false, true, true, false, false, true],
            "Right edge",
        );
        assert_eq!(
            tile.edge(Bottom),
            vec![false, false, true, true, true, false, false, true, true, true],
            "Bottom edge"
        );
        assert_eq!(
            tile.edge(Left),
            vec![false, true, true, true, true, true, false, false, true, false],
            "Left edge"
        );
    }

    #[test]
    fn transformed_edge() {
        use {Rotations::*, Side::*};

        let data: Vec<String> = strings(vec![
            "#.#.#.#.#.",
            "..........",
            "..........",
            "#.........",
            "..........",
            ".........#",
            "#........#",
            ".........#",
            ".........#",
            ".##.##.###",
        ]);
        let tile = Tile::new(0, &data);

        let it = iproduct!(
            [Left, Right, Top, Bottom].iter().cloned(),
            [Zero, Once, Twice, Thrice].iter().cloned(),
            [false, true].iter().cloned()
        );

        for (side, rot, flip) in it {
            let expected_str = match (side, rot, flip) {
                (Top, Zero, false) => "#.#.#.#.#.",
                (Top, Once, false) => "...#..#..#",
                (Top, Twice, false) => "###.##.##.",
                (Top, Thrice, false) => ".....#####",
                (Right, Zero, false) => ".....#####",
                (Right, Once, false) => "#.#.#.#.#.",
                (Right, Twice, false) => "...#..#..#",
                (Right, Thrice, false) => "###.##.##.",
                (Bottom, Zero, false) => ".##.##.###",
                (Bottom, Once, false) => "#####.....",
                (Bottom, Twice, false) => ".#.#.#.#.#",
                (Bottom, Thrice, false) => "#..#..#...",
                (Left, Zero, false) => "#..#..#...",
                (Left, Once, false) => ".##.##.###",
                (Left, Twice, false) => "#####.....",
                (Left, Thrice, false) => ".#.#.#.#.#",
                (Top, Zero, true) => ".#.#.#.#.#",
                (Top, Once, true) => "#####.....",
                (Top, Twice, true) => ".##.##.###",
                (Top, Thrice, true) => "#..#..#...",
                (Right, Zero, true) => "#..#..#...",
                (Right, Once, true) => ".#.#.#.#.#",
                (Right, Twice, true) => "#####.....",
                (Right, Thrice, true) => ".##.##.###",
                (Bottom, Zero, true) => "###.##.##.",
                (Bottom, Once, true) => "...#..#..#",
                (Bottom, Twice, true) => "#.#.#.#.#.",
                (Bottom, Thrice, true) => ".....#####",
                (Left, Zero, true) => ".....#####",
                (Left, Once, true) => "###.##.##.",
                (Left, Twice, true) => "...#..#..#",
                (Left, Thrice, true) => "#.#.#.#.#.",
            };

            let mut expected: Edge = vec![false; 10];
            for (i, c) in expected_str.chars().enumerate() {
                expected[i] = c == '#';
            }

            let actual = tile.transformed_edge(rot, flip, side);

            assert_eq!(actual, expected, "{:?}, {:?}, {:?}", side, rot, flip);
        }
    }

    #[test]
    fn placed_tile_data() {
        use Rotations::*;

        let data = strings(vec!["##..", ".#..", "....", "...."]);
        let tile = Tile::new(0, &data);

        let it = iproduct!(
            [Zero, Once, Twice, Thrice].iter().cloned(),
            [false, true].iter().cloned()
        );

        for (rot, flip) in it {
            let expected_data = strings(match (rot, flip) {
                (Zero, false) => vec!["##..", ".#..", "....", "...."],
                (Once, false) => vec!["...#", "..##", "....", "...."],
                (Twice, false) => vec!["....", "....", "..#.", "..##"],
                (Thrice, false) => vec!["....", "....", "##..", "#..."],
                (Zero, true) => vec!["..##", "..#.", "....", "...."],
                (Once, true) => vec!["....", "....", "..##", "...#"],
                (Twice, true) => vec!["....", "....", ".#..", "##.."],
                (Thrice, true) => vec!["#...", "##..", "....", "...."],
            });

            let expected = Tile::parse_strings(&expected_data);

            let actual = PlacedTile {
                tile: tile.clone(),
                rot,
                flip,
            }
            .transformed();

            assert_eq!(actual, expected, "{:?}, {:?}", rot, flip);
        }
    }

    fn strings(v: Vec<&str>) -> Vec<String> {
        let data: Vec<String> = v.iter().map(|s| s.to_string()).collect();
        data
    }
}
