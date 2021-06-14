/*
    Advent of Code 2020
    Caleb Stanford
    Day 24 Solution
    2020-12-24

    Time (--release): 0m0.255s
*/

use aoc2020::util::{file_to_vec, iter_rectangle};
use derive_more::{Add, Sum};
use std::collections::HashSet;
use std::iter::FromIterator;

/*
    Hexagonal coordinates

    These are just the same as rectangular coordinates with an appropriate
    choice of basis.
    The function agg_path aggregates the steps along a path for part 1.
    The iterators neighbors and iter_hex_box_padded are useful for part 2.
*/

#[derive(Add, Clone, Debug, Eq, Hash, PartialEq, Sum)]
struct HexCoord(isize, isize);

const E: HexCoord = HexCoord(1, 0);
const NE: HexCoord = HexCoord(0, 1);
const NW: HexCoord = HexCoord(-1, 1);
const W: HexCoord = HexCoord(-1, 0);
const SW: HexCoord = HexCoord(0, -1);
const SE: HexCoord = HexCoord(1, -1);
const ALL_DIRS: &[HexCoord] = &[E, NE, NW, W, SW, SE];

const HEXCOORD_MIN: HexCoord = HexCoord(isize::MIN, isize::MIN);
const HEXCOORD_MAX: HexCoord = HexCoord(isize::MAX, isize::MAX);

fn agg_path(path: &[HexCoord]) -> HexCoord {
    path.iter().cloned().sum()
}

// HexCoord iterators

fn neighbors(coord: &HexCoord) -> impl Iterator<Item = HexCoord> {
    // Need to clone coord to capture and use it in a closure
    let coord = coord.clone();
    ALL_DIRS.iter().cloned().map(move |dir| dir + coord.clone())
}

fn iter_hex_box_padded(
    bound_low: &HexCoord,
    bound_high: &HexCoord,
) -> impl Iterator<Item = HexCoord> {
    // Iterate over coordinates within hexagonal low/upper bounds, including
    // 1 layer of padding around the box
    let x0 = bound_low.0 - 1;
    let y0 = bound_low.1 - 1;
    let x1 = bound_high.0 + 1;
    let y1 = bound_high.1 + 1;
    iter_rectangle(x0, y0, x1, y1).map(|(x, y)| HexCoord(x, y))
}

/*
    Hexagonal grid

    For part 1: supports .toggle() to toggle tiles and FromIterator<HexCoord>
    to toggle all tiles specified in the input.

    For part 2: implements .step(), the game of life update rules.
*/

#[derive(Clone)]
struct HexGrid {
    grid: HashSet<HexCoord>,
    bound_low: HexCoord,
    bound_high: HexCoord,
}
impl HexGrid {
    fn new() -> Self {
        HexGrid {
            grid: HashSet::new(),
            bound_low: HEXCOORD_MAX,
            bound_high: HEXCOORD_MIN,
        }
    }
    fn len(&self) -> usize {
        self.grid.len()
    }

    // Core update functions
    fn update_bounds(&mut self, coord: &HexCoord) {
        self.bound_low.0 = self.bound_low.0.min(coord.0);
        self.bound_low.1 = self.bound_low.1.min(coord.1);
        self.bound_high.0 = self.bound_high.0.max(coord.0);
        self.bound_high.1 = self.bound_high.1.max(coord.1);
    }
    fn insert(&mut self, coord: HexCoord) {
        // Precondition: coord is not currently in grid
        debug_assert!(!self.grid.contains(&coord));
        self.update_bounds(&coord);
        self.grid.insert(coord);
    }
    fn toggle(&mut self, coord: &HexCoord) {
        // Makes sure to update bounds also
        if self.grid.contains(coord) {
            self.grid.remove(coord);
        } else {
            self.insert(coord.clone());
        }
    }

    // Game logic (for part 2)
    fn iter_coords(&self) -> impl Iterator<Item = HexCoord> {
        iter_hex_box_padded(&self.bound_low, &self.bound_high)
    }
    fn count_neighbors(&self, coord: &HexCoord) -> usize {
        neighbors(coord).map(|c| self.grid.contains(&c)).filter(|&b| b).count()
    }
    fn game_rule(&self, coord: &HexCoord) -> bool {
        // Return whether a tile is black in the next iteration
        let neighbors = self.count_neighbors(&coord);
        neighbors == 2 || neighbors == 1 && self.grid.contains(&coord)
    }
    fn step(&mut self) {
        let mut new_grid = Self::new();
        for coord in self.iter_coords() {
            if self.game_rule(&coord) {
                new_grid.insert(coord);
            }
        }
        *self = new_grid;
    }
    fn step_for(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step();
        }
    }
}
impl FromIterator<HexCoord> for HexGrid {
    fn from_iter<I: IntoIterator<Item = HexCoord>>(iter: I) -> Self {
        let mut grid = Self::new();
        for coord in iter {
            grid.toggle(&coord);
        }
        grid
    }
}

/*
    Input parsing

    This code is more verbose than I would like.
    I initially tried to use Regex for more concise parsing but it's not the
    best for this use case.
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

fn main() {
    let input = file_to_vec("input/day24.txt");
    let paths: Vec<Vec<_>> = parse_input(&input);
    // println!("Parsed input: {:?}", paths);

    println!("===== Part 1 =====");
    let grid: HexGrid = paths.iter().map(|p| agg_path(p)).collect();
    println!("Part 1 Answer: {}", grid.len());

    println!("===== Part 2 =====");
    let mut grid = grid;
    grid.step_for(100);
    println!("Part 2 Answer: {}", grid.len());
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
