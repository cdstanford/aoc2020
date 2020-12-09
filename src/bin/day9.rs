/*
    Advent of Code 2020
    Caleb Stanford
    Day 9 Solution
    2020-12-09
*/

use aoc2020::util::file_to_vec;
use std::collections::{HashMap, HashSet};

fn is_valid(prev_nums: &[isize], curr_num: isize) -> bool {
    assert!(prev_nums.len() == 25);
    let mut nums_set = HashSet::new();
    for &num in prev_nums {
        if nums_set.contains(&(curr_num - num)) {
            return true;
        }
        nums_set.insert(num);
    }
    false
}

fn solve_part1(nums: &[isize]) -> isize {
    for i in 25..nums.len() {
        let prev_nums = &nums[(i - 25)..i];
        if !is_valid(prev_nums, nums[i]) {
            return nums[i];
        }
    }
    unreachable!();
}

fn solve_part2(nums: &[isize], target: isize) -> isize {
    let mut partial_sums = HashMap::new();
    let mut sum = 0;
    for i in 0..nums.len() {
        partial_sums.insert(sum, i);
        sum += nums[i];
        if let Some(&j) = partial_sums.get(&(sum - target)) {
            // Set of numbers we are after is index j through i, inclusive
            let target_set: Vec<isize> = nums[j..(i + 1)].to_vec();
            let min = target_set.iter().min().unwrap();
            let max = target_set.iter().max().unwrap();
            return min + max;
        }
    }
    unreachable!();
}

fn main() {
    let nums: Vec<isize> = file_to_vec("input/day9.txt")
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    let ans_1 = solve_part1(&nums);
    let ans_2 = solve_part2(&nums, ans_1);
    println!("Part 1 Answer: {}", ans_1);
    println!("Part 2 Answer: {}", ans_2);
}
