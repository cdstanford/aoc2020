/*
    Advent of Code 2020
    Caleb Stanford
    Day 10 Solution
    2020-12-10
*/

use aoc2020::util::file_to_vec;

// General setup: sort the joltages and add min/max
fn preprocess_joltages(joltages: &mut Vec<usize>) {
    let low = 0;
    let high = joltages.iter().max().unwrap() + 3;
    joltages.push(low);
    joltages.push(high);
    joltages.sort_unstable();
}

// Part 1: output # of 1 diffs, # of 3 diffs
// Assumes joltages is sorted
fn get_differences(joltages: &[usize]) -> (usize, usize) {
    let mut ones = 0;
    let mut threes = 0;
    for i in 1..joltages.len() {
        match joltages[i] - joltages[i - 1] {
            1 => ones += 1,
            3 => threes += 1,
            _ => panic!(),
        }
    }
    (ones, threes)
}

// Part 2: count # of arrangements
// Assumes joltages is sorted
fn count_arrangements(joltages: &[usize]) -> usize {
    let mut counts = vec![1]; // # of arrangements ending in i
    for i in 1..joltages.len() {
        let mut new_count = 0;
        for j in 1..=i {
            if joltages[i] - joltages[i - j] <= 3 {
                new_count += counts[i - j];
            } else {
                break;
            }
        }
        counts.push(new_count);
    }
    assert_eq!(joltages.len(), counts.len());
    counts[counts.len() - 1]
}

// Tests
#[cfg(test)]
mod tests {
    use super::*;

    fn joltages_example() -> Vec<usize> {
        let mut joltages = vec![16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        preprocess_joltages(&mut joltages);
        joltages
    }

    #[test]
    fn test_get_differences() {
        assert_eq!(get_differences(&joltages_example()), (7, 5));
    }

    #[test]
    fn test_count_arrangements() {
        assert_eq!(count_arrangements(&joltages_example()), 8)
    }
}

fn main() {
    let mut joltages: Vec<usize> = file_to_vec("input/day10.txt")
        .iter()
        .map(|line| line.parse().unwrap())
        .collect();
    // Preprocess
    preprocess_joltages(&mut joltages);
    // Part 1
    let (ones, threes) = get_differences(&joltages);
    println!("Part 1 Answer: {}", ones * threes);
    // Part 2
    println!("Part 2 Answer: {}", count_arrangements(&joltages));
}
