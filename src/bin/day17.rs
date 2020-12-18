/*
    Advent of Code 2020
    Caleb Stanford
    Day 17 Solution
    2020-12-17

    Start time: 2:40pm
    Solved part 1: 3:40pm (1hr)
    Solved part 2: 3:58pm (18 min)
    Code cleanup: 6:00-7:25pm

    Time (--release): 0m0.374s
*/

use aoc2020::util::file_to_vec;
use std::collections::HashSet;

/*
    Abstractions for 4D coordinates.

    Initially I used 4-tuples, which can be more verbose but have the advantage
    of allowing for easier move semantics (here I have to use Copy or Clone to
    implement array_zip and array_map). Using fixed-size arrays generalizes
    better to vary the dimension.
    Another alternative would be the arrayvec crate, which provides better
    support for fixed-size arrays.
*/
type Coord = [isize; 4];
const COORD_MIN: Coord = [isize::MIN; 4];
const COORD_MAX: Coord = [isize::MAX; 4];
fn array_zip<T: Copy, U: Copy>(t1: &[T; 4], t2: &[U; 4]) -> [(T, U); 4] {
    [(t1[0], t2[0]), (t1[1], t2[1]), (t1[2], t2[2]), (t1[3], t2[3])]
}
fn array_map<T: Copy, U: Copy, F: Fn(T) -> U>(t: &[T; 4], f: F) -> [U; 4] {
    [f(t[0]), f(t[1]), f(t[2]), f(t[3])]
}
fn coordwise_min(c1: Coord, c2: Coord) -> Coord {
    array_map(&array_zip(&c1, &c2), |(i1, i2)| i1.min(i2))
}
fn coordwise_max(c1: Coord, c2: Coord) -> Coord {
    array_map(&array_zip(&c1, &c2), |(i1, i2)| i1.max(i2))
}
fn coordwise_shift(c: Coord, shift: isize) -> Coord {
    array_map(&c, |i| i + shift)
}
// Iterate over a multidimensional box of coordinates.
// This is very nice for avoiding nested for loops.
// This could be done a bit more idiomatically (but more verbosely) by defining
// a struct which implements Iterator<Item = Coord>.
fn do_for_box<F: FnMut(Coord)>(min_coord: Coord, max_coord: Coord, mut f: F) {
    for x in min_coord[0]..=max_coord[0] {
        for y in min_coord[1]..=max_coord[1] {
            for z in min_coord[2]..=max_coord[2] {
                for w in min_coord[3]..=max_coord[3] {
                    f([x, y, z, w]);
                }
            }
        }
    }
}

/*
    Data structure for an infinite 4D grid

    To solve both part 1 and 2, we include a 'dimension' parameter.
    Coordinates beyond the dimension are ignored (always 0).
*/
const MAX_DIMENSION: usize = 4;
#[derive(Clone, Debug)]
struct LifeGrid {
    active: HashSet<Coord>,
    min_coord: Coord,
    max_coord: Coord,
    dimension: usize,
}
impl LifeGrid {
    // Constructor and basic set functionality
    fn new(dimension: usize) -> Self {
        assert!(dimension <= MAX_DIMENSION);
        LifeGrid {
            active: HashSet::new(),
            min_coord: COORD_MAX,
            max_coord: COORD_MIN,
            dimension,
        }
    }
    fn is_active(&self, cell: Coord) -> bool {
        self.active.contains(&cell)
    }
    fn ok_for_dimension(&self, cell: Coord) -> bool {
        // Check if cell is within the bounds of the given dimension.
        cell.iter().skip(self.dimension).all(|&elem| elem == 0)
    }
    fn add_active(&mut self, cell: Coord) {
        assert!(self.ok_for_dimension(cell));
        self.min_coord = coordwise_min(self.min_coord, cell);
        self.max_coord = coordwise_max(self.max_coord, cell);
        self.active.insert(cell);
    }
    // Parse problem input
    fn parse_2d(lines: &[String], dimension: usize) -> Self {
        let mut grid = Self::new(dimension);
        for (i, row) in lines.iter().enumerate() {
            for (j, ch) in row.chars().enumerate() {
                if ch == '#' {
                    grid.add_active([i as isize, j as isize, 0, 0]);
                } else {
                    assert_eq!(ch, '.');
                }
            }
        }
        grid
    }
    // Implementation of the game rules
    fn count_neighbors_inclusive(&self, cell: Coord) -> usize {
        // This counts the whole 3 x 3 x 3 grid including cell
        let mut count = 0;
        let low = coordwise_shift(cell, -1);
        let high = coordwise_shift(cell, 1);
        do_for_box(low, high, |coord| {
            if self.is_active(coord) {
                count += 1;
            }
        });
        count
    }
    fn is_active_next(&self, cell: Coord) -> bool {
        self.ok_for_dimension(cell)
            && (self.count_neighbors_inclusive(cell) == 3
                || (self.is_active(cell)
                    && self.count_neighbors_inclusive(cell) == 4))
    }
    fn step(&mut self) {
        let mut new_grid = LifeGrid::new(self.dimension);
        let low = coordwise_shift(self.min_coord, -1);
        let high = coordwise_shift(self.max_coord, 1);
        do_for_box(low, high, |coord| {
            if self.is_active_next(coord) {
                new_grid.add_active(coord);
            }
        });
        *self = new_grid;
    }
    fn step_for(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step();
        }
    }
    // Answer
    fn count_active(&self) -> usize {
        self.active.len()
    }
}

fn solve_part1(input: &[String]) -> usize {
    let mut grid_3d = LifeGrid::parse_2d(input, 3);
    grid_3d.step_for(6);
    grid_3d.count_active()
}

fn solve_part2(input: &[String]) -> usize {
    let mut grid_4d = LifeGrid::parse_2d(input, 4);
    grid_4d.step_for(6);
    grid_4d.count_active()
}

fn main() {
    let input = file_to_vec("input/day17.txt");

    println!("Part 1 Answer: {}", solve_part1(&input));
    println!("Part 2 Answer: {}", solve_part2(&input));
}
