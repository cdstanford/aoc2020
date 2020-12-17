/*
    Advent of Code 2020
    Caleb Stanford
    Day 17 Solution
    2020-12-17

    Start time: 2:40pm
    Solved part 1: 3:40pm (1hr)
    Solved part 2:
    Code cleanup:

    Time (--release):
*/

use aoc2020::util::file_to_vec;
use std::collections::HashSet;

/*
    3D coordinates
*/
type Coord = (isize, isize, isize);
const COORD_MIN: Coord = (isize::MIN, isize::MIN, isize::MIN);
const COORD_MAX: Coord = (isize::MAX, isize::MAX, isize::MAX);
fn tuple_zip<T, U>(t1: (T, T, T), t2: (U, U, U)) -> ((T, U), (T, U), (T, U)) {
    ((t1.0, t2.0), (t1.1, t2.1), (t1.2, t2.2))
}
fn tuple_map<T, U, F>(t: (T, T, T), f: F) -> (U, U, U)
where
    F: Fn(T) -> U,
{
    (f(t.0), f(t.1), f(t.2))
}
fn coordwise_min(c1: Coord, c2: Coord) -> Coord {
    tuple_map(tuple_zip(c1, c2), |(i1, i2)| i1.min(i2))
}
fn coordwise_max(c1: Coord, c2: Coord) -> Coord {
    tuple_map(tuple_zip(c1, c2), |(i1, i2)| i1.max(i2))
}

/*
    Infinite 3D Grid
*/
#[derive(Clone, Debug)]
struct LifeGrid {
    active: HashSet<Coord>,
    min_coord: Coord,
    max_coord: Coord,
}
impl LifeGrid {
    // Constructor and basic set functionality
    fn new() -> Self {
        LifeGrid {
            active: HashSet::new(),
            min_coord: COORD_MAX,
            max_coord: COORD_MIN,
        }
    }
    fn is_active(&self, cell: Coord) -> bool {
        self.active.contains(&cell)
    }
    fn add_active(&mut self, cell: Coord) {
        if !self.is_active(cell) {
            self.min_coord = coordwise_min(self.min_coord, cell);
            self.max_coord = coordwise_max(self.max_coord, cell);
            self.active.insert(cell);
        }
    }
    // Parse problem input
    fn parse_2d(lines: Vec<String>) -> Self {
        let mut grid = Self::new();
        for (i, row) in lines.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                match ch {
                    '#' => {
                        grid.add_active((i as isize, j as isize, 0));
                    }
                    '.' => (),
                    _ => unreachable!(),
                }
            }
        }
        grid
    }
    // Implementation of the game rules
    fn count_neighbors_inclusive(&self, cell: Coord) -> usize {
        // This counts the whole 3 x 3 x 3 grid including cell
        let mut count = 0;
        for &x in &[cell.0 - 1, cell.0, cell.0 + 1] {
            for &y in &[cell.1 - 1, cell.1, cell.1 + 1] {
                for &z in &[cell.2 - 1, cell.2, cell.2 + 1] {
                    if self.is_active((x, y, z)) {
                        count += 1;
                    }
                }
            }
        }
        count
    }
    fn is_active_next(&self, cell: Coord) -> bool {
        self.count_neighbors_inclusive(cell) == 3
            || (self.is_active(cell)
                && self.count_neighbors_inclusive(cell) == 4)
    }
    fn step(&mut self) {
        let mut new_grid = LifeGrid::new();
        for x in (self.min_coord.0 - 1)..(self.max_coord.0 + 2) {
            for y in (self.min_coord.1 - 1)..(self.max_coord.1 + 2) {
                for z in (self.min_coord.2 - 1)..(self.max_coord.2 + 2) {
                    let coord = (x, y, z);
                    if self.is_active_next(coord) {
                        new_grid.add_active(coord);
                    }
                }
            }
        }
        *self = new_grid;
    }
    fn step_for(&mut self, iterations: usize) {
        for _i in 0..iterations {
            self.step();
        }
    }
    // Answer
    fn count_active(&self) -> usize {
        self.active.len()
    }
}

fn solve_part1(mut grid: LifeGrid) -> usize {
    grid.step_for(6);
    grid.count_active()
}

fn solve_part2(mut _grid: LifeGrid) -> usize {
    0
}

fn main() {
    let input = file_to_vec("input/day17.txt");
    let grid = LifeGrid::parse_2d(input);

    println!("Grid: {:?}", grid);

    println!("Part 1 Answer: {}", solve_part1(grid.clone()));
    println!("Part 2 Answer: {}", solve_part2(grid));
}
