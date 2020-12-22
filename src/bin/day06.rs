/*
    Advent of Code 2020
    Caleb Stanford
    Day 6 Solution
    2020-12-07
*/

use aoc2020::util::file_to_vec_el;
use std::collections::HashSet;

// Part 1: Total that at least one member in group answered yes
fn some_yes(group: &[String]) -> usize {
    group.iter().flat_map(|s| s.chars()).collect::<HashSet<char>>().len()
}

// Part 2: Total # in group that every member answered yes
fn all_yes(group: &[String]) -> usize {
    let alphabet: HashSet<char> =
        "abcdefghijklmnopqrstuvwxyz".chars().collect();
    group
        .iter()
        .map(|s| s.chars().collect::<HashSet<char>>())
        .fold(alphabet, |x, y| x.intersection(&y).cloned().collect())
        .len()
}

fn solve_part1(data: &[Vec<String>]) -> usize {
    data.iter().map(|group| some_yes(&group)).sum()
}

fn solve_part2(data: &[Vec<String>]) -> usize {
    data.iter().map(|group| all_yes(&group)).sum()
}

fn main() {
    let mut data = Vec::new();
    let mut group = Vec::new();
    for line in file_to_vec_el("input/day06.txt") {
        if line == "" {
            data.push(group);
            group = Vec::new();
        } else {
            group.push(line);
        }
    }
    // println!("Input: {:?}", data);
    println!("Part 1 Answer: {}", solve_part1(&data));
    println!("Part 2 Answer: {}", solve_part2(&data));
}
