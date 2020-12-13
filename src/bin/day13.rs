/*
    Advent of Code 2020
    Caleb Stanford
    Day 13 Solution
    2020-12-13
*/

use aoc2020::util::file_to_vec;

// Return the smallest multiple of n >= target
fn smallest_multiple(n: usize, target: usize) -> usize {
    n * ((target + n - 1) / n)
}

fn solve_part1(target: usize, buses: &[Option<usize>]) -> usize {
    let (bus, time) = buses
        .iter()
        .filter(|&&bus| bus != None)
        .map(|bus| bus.unwrap())
        .map(|bus| {
            let multiple = smallest_multiple(bus, target);
            println!("    Bus {}: smallest multiple {}", bus, multiple);
            (bus, multiple)
        })
        .min_by_key(|(_bus, time)| *time)
        .unwrap();
    println!("Best: bus {} with delay {}", bus, time - target);
    bus * (time - target)
}

// Chinese remainder theorem implementation.
// Assumes mod1 and mod2 are relatively prime and returns the unique remainder
// mod (mod1 * mod2)
fn chinese_remainder(
    rem1: usize,
    mod1: usize,
    rem2: usize,
    mod2: usize,
) -> usize {
    if mod1 < mod2 {
        chinese_remainder(rem2, mod2, rem1, mod1)
    } else {
        let mut rem = rem1;
        while (rem % mod2) != rem2 {
            rem += mod1;
        }
        rem
    }
}

// True modulus function that works for negative numbers
fn modulo(num: isize, modulus: usize) -> usize {
    let modulus = modulus as isize;
    let result = ((num % modulus) + modulus) % modulus;
    result as usize
}

fn solve_part2(buses: &[Option<usize>]) -> usize {
    let (rem, _modulus) = buses
        .iter()
        .enumerate()
        .filter(|&(_i, &bus)| bus != None)
        .map(|(i, bus)| (i, bus.unwrap()))
        .map(|(i, bus)| (modulo(-(i as isize), bus), bus))
        .fold((0, 1), |(rem1, mod1), (rem2, mod2)| {
            println!(
                "    Bus {}: folding ({}, {}), ({}, {})",
                mod2, rem1, mod1, rem2, mod2
            );
            (chinese_remainder(rem1, mod1, rem2, mod2), mod1 * mod2)
        });
    rem
}

fn main() {
    let input = file_to_vec("input/day13.txt");
    let target: usize = input[0].parse().unwrap();
    let buses: Vec<Option<usize>> = input[1]
        .split(',')
        .map(|s| if s == "x" { None } else { Some(s.parse().unwrap()) })
        .collect();

    // println!("Target: {}", target);
    // println!("Buses: {:?}", buses);

    println!("Part 1 Answer: {}", solve_part1(target, &buses));
    println!("Part 2 Answer: {}", solve_part2(&buses));
}
