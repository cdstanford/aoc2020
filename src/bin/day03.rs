/*
    Advent of Code 2020
    Caleb Stanford
    Day 3 Solution
    2020-12-06
*/

use aoc2020::util::file_to_vec;

use std::fmt::{Display, Formatter, Result};

#[derive(Clone, Debug)]
struct TobogganMap {
    rows: usize,
    cols: usize,
    grid: Vec<Vec<bool>>,
}
impl TobogganMap {
    /* Get map from input lines of '#' and '.' */
    fn parse_input(input: Vec<String>) -> Self {
        let rows = input.len();
        assert!(rows > 0);
        let cols = input[0].len();

        let parse_char = |ch| match ch {
            '#' => true,
            '.' => false,
            _ => panic!(),
        };
        let parse_row = |row: &String| {
            assert_eq!(row.len(), cols);
            row.chars().map(parse_char).collect()
        };
        let grid = input.iter().map(parse_row).collect();

        Self { rows, cols, grid }
    }

    /* Iterate over a toboggan route */
    fn path(
        &self,
        down: usize,
        right: usize,
    ) -> impl Iterator<Item = bool> + '_ {
        let mut col = 0;
        (0..(self.rows)).step_by(down).map(move |row| {
            let result = self.grid[row][col];
            col = (col + right) % self.cols;
            result
        })
    }
    /* Count the trees along a toboggan route */
    fn count_trees(&self, down: usize, right: usize) -> usize {
        self.path(down, right).filter(|&x| x).count()
    }
}
impl Display for TobogganMap {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        for row in &self.grid {
            for &col in row {
                let ch = if col { '#' } else { '.' };
                write!(f, "{}", ch)?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn solve_part1(tob_map: &TobogganMap) -> usize {
    tob_map.count_trees(1, 3)
}

fn solve_part2(tob_map: &TobogganMap) -> usize {
    let slopes = &[(1, 1), (1, 3), (1, 5), (1, 7), (2, 1)];
    slopes.iter().map(|&(x, y)| tob_map.count_trees(x, y)).product()
}

fn main() {
    let raw_input = file_to_vec("input/day03.txt");
    let tob_map = TobogganMap::parse_input(raw_input);
    // println!("{}", tob_map);
    println!("Part 1 Answer: {}", solve_part1(&tob_map));
    println!("Part 2 Answer: {}", solve_part2(&tob_map));
}
