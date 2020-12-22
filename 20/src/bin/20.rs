#![feature(or_patterns)]

use aoc20::util::{parse, print_answers};
use bitvec::prelude::*;
use itertools::iproduct;
use num::integer::Roots;
use regex::Regex;
use std::{
    convert::TryInto,
    hash::{Hash, Hasher},
    cmp::min,
    collections::{HashMap, HashSet},
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

    fn n(&self) -> u64 {
        match self {
            Side::Top => 0,
            Side::Right => 1,
            Side::Bottom => 2,
            Side::Left => 3,
        }
    }
}

type Edge = bitarr!(for 10, in Lsb0, u16);

type TileId = u64;

#[derive(Clone, Hash, Debug)]
struct Tile {
    id: TileId,
    bits: bitarr!(for 100),
}

impl Tile {
    fn new(id: TileId, strings: &[String]) -> Tile {
        let mut bits = bitarr![0; 100];
        for (y, line) in strings.iter().enumerate() {
            for (x, c) in line.chars().enumerate() {
                if c == '#' {
                    bits.set(y * 10 + x, true);
                }
            }
        }
        Tile { id, bits }
    }

    fn edge(&self, side: Side) -> Edge {
        match side {
            Side::Top => {
                let mut bits = Edge::zeroed();
                bits[0..10].for_each(|idx, _| self.bits[idx]);
                bits
            }
            Side::Right => {
                let mut bits = Edge::zeroed();
                bits[0..10].for_each(|idx, _| self.bits[idx * 10 + 9]);
                bits
            }
            Side::Bottom => {
                let mut bits = Edge::zeroed();
                bits[0..10].for_each(|idx, _| self.bits[idx + 90]);
                bits
            }
            Side::Left => {
                let mut bits = Edge::zeroed();
                bits[0..10].for_each(|idx, _| self.bits[idx * 10]);
                bits
            }
        }
    }

    fn transformed_edge(&self, rot: Rotations, flip: bool, side: Side) -> Edge {
        use {Rotations::*, Side::*};

        let rev = |mut e: Edge| -> Edge {
            e[0..10].reverse();
            e
        };

        let cw = |side: Side, rot: Rotations| Side::at(side.n() + 4 - rot.n());

        match (side, rot, flip) {
            (Top | Bottom, Zero, false) => self.edge(side),
            (Top | Bottom, Once, false) => rev(self.edge(cw(side, rot))),
            (Top | Bottom, Twice, false) => rev(self.edge(cw(side, rot))),
            (Top | Bottom, Thrice, false) => self.edge(cw(side, rot)),
            (Right | Left, Zero, false) => self.edge(side),
            (Right | Left, Once, false) => self.edge(cw(side, rot)),
            (Right | Left, Twice, false) => rev(self.edge(cw(side, rot))),
            (Right | Left, Thrice, false) => rev(self.edge(cw(side, rot))),
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

impl Rotations {
    fn n(&self) -> u64 {
        match self {
            Rotations::Zero => 0,
            Rotations::Once => 1,
            Rotations::Twice => 2,
            Rotations::Thrice => 3,
        }
    }
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

fn build_edge_lookup(tiles: &HashMap<TileId, Tile>) -> HashMap<u16, HashSet<TileId>> {
    let mut edges_tiles: HashMap<u16, HashSet<TileId>> = HashMap::new();
    for block in tiles.values() {
        for side in (0..4).map(Side::at) {
            let edge = block.edge(side);

            let k = key(edge);

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

fn key(edge: Edge) -> u16 {
    let rev = {
        let mut bits = edge.clone();
        bits[0..10].reverse();
        bits
    };
    let k = min(edge.unwrap()[0], rev.unwrap()[0]);
    k
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

    let corners = corners(tiles);
    let edge_lookup = build_edge_lookup(tiles);
    let is_unique = |edge: Edge| -> bool { edge_lookup.get(&key(edge)).unwrap().len() == 1 };
    let unique_edge_counts = unique_edge_counts(tiles);

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
                    .get(&key(to_match))
                    .map(|set| set.iter())
                    .and_then(|it| {
                        it.filter(|&id| tiles.contains_key(id))
                            .filter(|&id| !is_tried(id))
                            .next()
                    })
            } else if i % 12 == 0 {
                let to_match = arrangement[i - 12].edge(Bottom);
                edge_lookup
                    .get(&key(to_match))
                    .map(|set| set.iter())
                    .and_then(|it| {
                        it.filter(|&id| tiles.contains_key(id))
                            .filter(|&id| !is_tried(id))
                            .next()
                    })
            } else {
                let to_match = (
                    arrangement[i - 1].edge(Right),
                    arrangement[i - 12].edge(Bottom),
                );
                let intersection = edge_lookup
                    .get(&key(to_match.0))
                    .and_then(|set1| {
                        edge_lookup
                            .get(&key(to_match.1))
                            .map(|set2| set1.intersection(set1))
                    })
                    .unwrap();
                intersection
                    .filter(|&id| tiles.contains_key(&id))
                    .filter(|&id| !is_tried(id))
                    .next()
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

    todo!()
}

fn hash(arrangement: &Vec<PlacedTile>) -> u64 {
    let h = &mut std::collections::hash_map::DefaultHasher::new();
    arrangement.hash(h);
    h.finish()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn edges() {
        let v: Vec<String> = vec![
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
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let block = Tile::new(0, &v);

        use Side::*;
        assert_eq!(
            block.edge(Top)[0..10],
            bits![0, 0, 1, 1, 0, 1, 0, 0, 1, 0],
            "Top edge"
        );
        assert_eq!(
            block.edge(Right)[0..10],
            bits![0, 0, 0, 1, 0, 1, 1, 0, 0, 1],
            "Right edge",
        );
        assert_eq!(
            block.edge(Bottom)[0..10],
            bits![0, 0, 1, 1, 1, 0, 0, 1, 1, 1],
            "Bottom edge"
        );
        assert_eq!(
            block.edge(Left)[0..10],
            bits![0, 1, 1, 1, 1, 1, 0, 0, 1, 0],
            "Left edge"
        );
    }

    #[test]
    fn transformed_edge() {
        use {Rotations::*, Side::*};

        let data: Vec<String> = vec![
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
        ]
            .iter()
            .map(|s| s.to_string())
            .collect();
        let tile = Tile::new(0, &data);

        let it = || {
            iproduct!(
                [Left, Right, Top, Bottom].iter().cloned(),
                [Zero, Once, Twice, Thrice].iter().cloned(),
                [false, true].iter().cloned()
            )
        };

        for (side, rot, flip) in it() {
            let expected = match (side, rot, flip) {
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

            let mut expected_bits = Edge::zeroed();
            expected_bits[0..10].for_each(|idx, _| expected.chars().nth(idx).unwrap() == '#');

            let actual_bits = tile.transformed_edge(rot, flip, side);

            assert_eq!(
                actual_bits, expected_bits,
                "{:?}, {:?}, {:?}",
                side, rot, flip
            );
        }
    }
}

