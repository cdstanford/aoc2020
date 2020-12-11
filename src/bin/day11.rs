/*
    Advent of Code 2020
    Caleb Stanford
    Day 11 Solution
    2020-12-11
*/

use aoc2020::util::file_to_vec;
use std::fmt;

const DIRECTIONS: &[(isize, isize)] =
    &[(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)];

#[derive(Clone, Debug)]
struct SeatMap {
    rows: usize,
    cols: usize,
    seats: Vec<Vec<char>>, // padded: dimensions (rows + 2) x (col + 2)
    use_sight_rules: bool, // use line of sight rules (for part 2)
}
impl SeatMap {
    fn new(seats_unpadded: &[String], use_sight_rules: bool) -> Self {
        debug_assert!(!seats_unpadded.is_empty());
        let rows = seats_unpadded.len();
        let cols = seats_unpadded[0].chars().count();
        // Pad seats
        let mut seats = Vec::new();
        let row_pad = vec!['.'; cols + 2];
        seats.push(row_pad.clone());
        for seat in seats_unpadded {
            seats.push((".".to_owned() + seat + ".").chars().collect());
        }
        seats.push(row_pad);
        Self { rows, cols, seats, use_sight_rules }
    }
    fn adjacent_seat(
        &self,
        row: usize,
        col: usize,
        drow: isize,
        dcol: isize,
    ) -> char {
        let adj_row = row as isize + drow;
        let adj_col = col as isize + dcol;
        self.seats[adj_row as usize][adj_col as usize]
    }
    fn seen_seat(
        &self,
        row: usize,
        col: usize,
        drow: isize,
        dcol: isize,
    ) -> char {
        let mut see_row = row as isize;
        let mut see_col = col as isize;
        loop {
            see_row += drow;
            see_col += dcol;
            if see_row == 0
                || see_row == self.rows as isize + 1
                || see_col == 0
                || see_col == self.cols as isize + 1
            {
                return '.';
            }
            let seat = self.seats[see_row as usize][see_col as usize];
            if seat != '.' {
                return seat;
            }
        }
    }
    fn neighbor_seats(&self, row: usize, col: usize) -> Vec<char> {
        debug_assert!(row >= 1 && row <= self.rows);
        debug_assert!(col >= 1 && col <= self.cols);
        let mut result = Vec::new();
        if self.use_sight_rules {
            for &(drow, dcol) in DIRECTIONS {
                result.push(self.seen_seat(row, col, drow, dcol));
            }
        } else {
            for &(drow, dcol) in DIRECTIONS {
                result.push(self.adjacent_seat(row, col, drow, dcol));
            }
        }
        debug_assert_eq!(result.len(), 8);
        result
    }
    fn neighbors_occupied(&self, row: usize, col: usize) -> usize {
        debug_assert!(row >= 1 && row <= self.rows);
        debug_assert!(col >= 1 && col <= self.cols);
        self.neighbor_seats(row, col).iter().filter(|&&ch| ch == '#').count()
    }
    fn tolerance(&self) -> usize {
        // Number of adjacent occupied seats that are tolerated
        if self.use_sight_rules {
            5
        } else {
            4
        }
    }
    fn new_seat(&self, row: usize, col: usize) -> char {
        debug_assert!(row >= 1 && row <= self.rows);
        debug_assert!(col >= 1 && col <= self.cols);
        let old_seat = self.seats[row][col];
        let adj_occupied = self.neighbors_occupied(row, col);
        if old_seat == 'L' && adj_occupied == 0 {
            '#'
        } else if old_seat == '#' && adj_occupied >= self.tolerance() {
            'L'
        } else {
            old_seat
        }
    }
    fn step(&mut self) -> bool {
        // true if changed
        let mut new_seats = self.seats.clone();
        #[allow(clippy::needless_range_loop)]
        for row in 1..=self.rows {
            for col in 1..=self.cols {
                new_seats[row][col] = self.new_seat(row, col);
            }
        }
        let changed = self.seats != new_seats;
        self.seats = new_seats;
        changed
    }
    fn step_until_stable(&mut self) {
        let mut count = 0;
        while self.step() {
            // Uncomment to print seat map as it steps
            // println!("{}", seat_map);
            // println!("-----");
            count += 1;
        }
        println!("[reached stable after {} steps]", count);
    }
    fn count_occupied(&self) -> usize {
        self.seats
            .iter()
            .flat_map(|row| row.iter())
            .filter(|&&ch| ch == '#')
            .count()
    }
}
impl fmt::Display for SeatMap {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for row in &self.seats {
            let row_string: String = row.iter().collect();
            writeln!(f, "{}", row_string)?
        }
        Ok(())
    }
}

fn solve_part1(lines: &[String]) -> usize {
    let mut seat_map = SeatMap::new(&lines, false);
    seat_map.step_until_stable();
    seat_map.count_occupied()
}

fn solve_part2(lines: &[String]) -> usize {
    let mut seat_map = SeatMap::new(&lines, true);
    seat_map.step_until_stable();
    seat_map.count_occupied()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_input() {
        let lines = file_to_vec("input/day11_test.txt");
        assert_eq!(solve_part1(&lines), 37);
        assert_eq!(solve_part2(&lines), 26);
    }
}

fn main() {
    let lines = file_to_vec("input/day11.txt");
    println!("Part 1 Answer: {}", solve_part1(&lines));
    println!("Part 2 Answer: {}", solve_part2(&lines));
}
