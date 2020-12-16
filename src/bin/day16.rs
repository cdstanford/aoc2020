/*
    Advent of Code 2020
    Caleb Stanford
    Day 16 Solution
    2020-12-16

    Start time: 11:41am
    Solved part 1: 1:41pm (2hrs)
    Solved part 2:
    Code cleanup:

    Time (--release):
*/

use aoc2020::util::{file_to_vec, iter_to_pair};
use std::fmt;

/*
    Inspecting the input, all numbers are small (between 1 and 999), and the
    range constraint boundaries are statically known.
    Therefore the best way to store range constraints (unions of ranges) should
    just be a vector<bool> of length 1000, not something fancier like a sorted
    list of the range boundaries.
*/
const GLOBAL_UB: usize = 1000;
struct Ranges {
    set: [bool; GLOBAL_UB],
}
impl Ranges {
    // Constructors
    fn new_empty() -> Self {
        Self { set: [false; GLOBAL_UB] }
    }
    fn from_range(low: usize, high: usize) -> Self {
        // Inclusive
        let mut result = Self::new_empty();
        for i in low..=high {
            result.set[i] = true;
        }
        result
    }
    // Membership check
    fn contains(&self, i: usize) -> bool {
        debug_assert!(i < GLOBAL_UB);
        self.set[i]
    }
    // Combining ranges (immutably)
    fn union(&self, other: &Self) -> Self {
        let mut result = Self::new_empty();
        for i in 0..GLOBAL_UB {
            result.set[i] = self.contains(i) || other.contains(i)
        }
        result
    }
}
// Need to implement Debug for printing arrays because of no const generics yet
impl fmt::Debug for Ranges {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for &val_bool in &self.set[..] {
            let val_str = if val_bool { "1" } else { "0" };
            f.write_str(val_str)?;
        }
        Ok(())
    }
}

/*
    Solutions
*/
fn invalid_fields(ticket: &[usize], constraints: &Ranges) -> Vec<usize> {
    ticket.iter().filter(|&&n| !constraints.contains(n)).cloned().collect()
}
fn solve_part1(fields: &[(String, Ranges)], tickets: &[Vec<usize>]) -> usize {
    let constraints = fields
        .iter()
        .map(|(_field_name, ranges)| ranges)
        .fold(Ranges::new_empty(), |r1, r2| r1.union(r2));

    tickets.iter().flat_map(|ticket| invalid_fields(ticket, &constraints)).sum()
}
fn solve_part2(
    _fields: &[(String, Ranges)],
    _tickets: &[Vec<usize>],
    _your_ticket: &[usize],
) -> usize {
    0
}

/*
    Parsing help
*/
fn parse_field(line: &str) -> (String, Ranges) {
    let (field_name, split0) = iter_to_pair(line.split(": "));
    let (split1, split2) = iter_to_pair(split0.split(" or "));
    let (low1, high1) =
        iter_to_pair(split1.split('-').map(|n| n.parse().unwrap()));
    let (low2, high2) =
        iter_to_pair(split2.split('-').map(|n| n.parse().unwrap()));

    let range1 = Ranges::from_range(low1, high1);
    let range2 = Ranges::from_range(low2, high2);
    let ranges = range1.union(&range2);

    (field_name.to_owned(), ranges)
}
fn parse_ticket(line: &str) -> Vec<usize> {
    let result: Vec<usize> =
        line.split(',').map(|n| n.parse().unwrap()).collect();
    assert_eq!(result.len(), 20);
    result
}

/*
    Entrypoint
*/
fn main() {
    let lines = file_to_vec("input/day16.txt");

    let fields: Vec<(String, Ranges)> =
        lines[0..20].iter().map(|s| s as &str).map(parse_field).collect();
    assert_eq!(fields.len(), 20);

    assert_eq!(lines[20], "");
    assert_eq!(lines[21], "your ticket:");
    let your_ticket: Vec<usize> = parse_ticket(&lines[22]);

    assert_eq!(lines[23], "");
    assert_eq!(lines[24], "nearby tickets:");
    let tickets: Vec<Vec<usize>> =
        lines[25..].iter().map(|s| s as &str).map(parse_ticket).collect();

    println!("Part 1 Answer: {}", solve_part1(&fields, &tickets));
    println!("Part 2 Answer: {}", solve_part2(&fields, &tickets, &your_ticket));
}
