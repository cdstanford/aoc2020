/*
    Advent of Code 2020
    Caleb Stanford
    Day 5 Solution
    2020-12-07
*/

use aoc2020::util::file_to_vec;

fn seat_id(board_pass: &str) -> usize {
    let mut seat = 0;
    for ch in board_pass.chars() {
        match ch {
            'F' | 'L' => {
                seat *= 2;
            }
            'B' | 'R' => {
                seat = 2 * seat + 1;
            }
            _ => panic!(format!(
                "invalid character {} in boarding pass: {}",
                ch, board_pass
            )),
        };
    }
    seat
}

fn triangle_number(n: usize) -> usize {
    n * (n + 1) / 2
}

/* Tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sead_id() {
        assert_eq!(seat_id("BFFFBBFRRR"), 567);
        assert_eq!(seat_id("FFFBBBFRRR"), 119);
        assert_eq!(seat_id("BBFFBBFRLL"), 820);
    }
}

/* Solutions */

fn solve_part1(board_passes: &[String]) -> usize {
    board_passes.iter().map(|s| seat_id(s.as_ref())).max().unwrap()
}

fn solve_part2(board_passes: &[String]) -> usize {
    let min = board_passes.iter().map(|s| seat_id(s.as_ref())).min().unwrap();
    let max = board_passes.iter().map(|s| seat_id(s.as_ref())).max().unwrap();
    let total: usize = board_passes.iter().map(|s| seat_id(s.as_ref())).sum();
    // Formula for the answer
    triangle_number(max) - triangle_number(min) + min - total
}

fn main() {
    let board_passes = file_to_vec("input/day5.txt");
    println!("Part 1 Answer: {}", solve_part1(&board_passes));
    println!("Part 2 Answer: {}", solve_part2(&board_passes));
}
