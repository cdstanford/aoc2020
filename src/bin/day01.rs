/*
    Advent of Code 2020
    Caleb Stanford
    Day 1 Solution
    2020-12-05
*/

use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::string::String;
use std::vec::Vec;

/* Util */

fn file_to_vec(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|x| x.unwrap()).collect()
}
fn file_to_int_vec(filepath: &str) -> Vec<usize> {
    file_to_vec(filepath)
        .into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect()
}

/* Solution */

fn find_sum2(nums: &[usize], target: usize) -> (usize, usize) {
    let mut seen = HashSet::new();
    for &num in nums {
        if seen.contains(&(target - num)) {
            return (target - num, num);
        }
        seen.insert(num);
    }
    panic!("Did not find sum :(");
}

fn find_sum3(nums: &[usize], target: usize) -> (usize, usize, usize) {
    let mut seen_sums = HashMap::new();
    for &x1 in nums {
        for &x2 in nums {
            seen_sums.insert(x1 + x2, (x1, x2));
        }
    }
    for &x3 in nums {
        if let Some(&(x1, x2)) = seen_sums.get(&(target - x3)) {
            return (x1, x2, x3);
        }
    }
    panic!("Did not find sum :(");
}

fn main() {
    let nums = file_to_int_vec("input/day01.txt");
    // println!("Nums: {:?}", nums);

    /* Part 1 */
    let (x1, x2) = find_sum2(&nums, 2020);
    println!("Part 1 Answer: {} * {} = {}", x1, x2, x1 * x2);

    /* Part 2 */
    let (x1, x2, x3) = find_sum3(&nums, 2020);
    println!("Part 2 Answer: {} * {} * {} = {}", x1, x2, x3, x1 * x2 * x3);
}
