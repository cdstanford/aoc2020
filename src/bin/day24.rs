/*
    Advent of Code 2020
    Caleb Stanford
    Day 24 Solution
    2020-12-24

    Time (--release):
*/

use aoc2020::util::file_to_vec;
use derive_more::{Add, Sum};
use std::collections::HashSet;
use std::iter::FromIterator;

/*
    Hexagonal coordinates

    These are just the same as rectangular coordinates with an appropriate
    choice of basis.
    The function sum_path aggregates the steps along a path.
*/

#[derive(Add, Clone, Debug, Eq, Hash, PartialEq, Sum)]
struct HexCoord(isize, isize);

const E: HexCoord = HexCoord(1, 0);
const NE: HexCoord = HexCoord(0, 1);
const NW: HexCoord = HexCoord(-1, 1);
const W: HexCoord = HexCoord(-1, 0);
const SW: HexCoord = HexCoord(0, -1);
const SE: HexCoord = HexCoord(1, -1);

fn agg_path(path: &[HexCoord]) -> HexCoord {
    let result = path.iter().cloned().sum();
    result
}

/*
    Hexagonal grid

    For part 1: supports .toggle() to toggle tiles and FromIterator<HexCoord>
    to toggle all tiles specified in the input.
*/

struct HexGrid(HashSet<HexCoord>);
impl HexGrid {
    fn new() -> Self {
        HexGrid(HashSet::new())
    }
    fn toggle(&mut self, coord: HexCoord) {
        if self.0.contains(&coord) {
            self.0.remove(&coord);
        } else {
            self.0.insert(coord);
        }
    }
}
impl FromIterator<HexCoord> for HexGrid {
    fn from_iter<I: IntoIterator<Item = HexCoord>>(iter: I) -> Self {
        let mut grid = Self::new();
        for coord in iter {
            grid.toggle(coord);
        }
        grid
    }
}

/*
    Input parsing
*/

fn parse_dir(dir_raw: &str) -> HexCoord {
    match dir_raw {
        "e" => E,
        "ne" => NE,
        "nw" => NW,
        "w" => W,
        "sw" => SW,
        "se" => SE,
        _ => panic!("Could not parse direction: {}", dir_raw),
    }
}
fn parse_line(line: &str) -> Vec<HexCoord> {
    let mut char_iter = line.chars();
    let mut result = Vec::new();
    loop {
        let ch1 = char_iter.next();
        if ch1.is_none() {
            return result;
        }
        let mut raw = ch1.unwrap().to_string();
        if raw != "e" && raw != "w" {
            raw.push(char_iter.next().unwrap());
        }
        result.push(parse_dir(&raw));
    }
}
fn parse_input(lines: &[String]) -> Vec<Vec<HexCoord>> {
    lines.iter().map(|s| parse_line(s)).collect()
}

/*
    Solutions and entrypoint
*/

fn solve_part1(paths: &[Vec<HexCoord>]) -> usize {
    let grid: HexGrid = paths.iter().map(|p| agg_path(p)).collect();
    grid.0.len()
}

fn solve_part2(_paths: &[Vec<HexCoord>]) -> usize {
    0
}

fn main() {
    let input = file_to_vec("input/day24.txt");
    let paths: Vec<Vec<_>> = parse_input(&input);
    // println!("Parsed input: {:?}", paths);

    println!("Part 1 Answer: {}", solve_part1(&paths));
    println!("Part 2 Answer: {}", solve_part2(&paths));
}

/*
    Unit tests
*/

#[cfg(test)]
mod tests {
    use super::*;

    const ORIGIN: HexCoord = HexCoord(0, 0);

    #[test]
    fn test_directions() {
        assert_eq!(E + W, ORIGIN);
        assert_eq!(NE + SW, ORIGIN);
        assert_eq!(NW + SE, ORIGIN);
        assert_eq!(E + NW, NE);
        assert_eq!(NE + W, NW);
        assert_eq!(NW + SW, W);
        assert_eq!(W + SE, SW);
        assert_eq!(SW + E, SE);
        assert_eq!(SE + NE, E);
    }
}
