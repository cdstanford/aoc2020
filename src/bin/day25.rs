/*
    Advent of Code 2020
    Caleb Stanford
    Day 25 Solution
    2020-12-26

    Time (--release): 0m0.166s
*/

use aoc2020::util::{file_to_vec_parsed, iter_to_pair};

// Fixed prime number modulus for the problem
const MODULUS: usize = 20201227;

fn encrypt(mut base: usize, mut pow: usize) -> usize {
    // Calculate the result of base^pow (mod MODULUS).
    // Uses repeated squaring.
    let mut result = 1;
    base %= MODULUS;
    while pow > 0 {
        if pow % 2 == 1 {
            result *= base;
            result %= MODULUS;
        }
        base = (base * base) % MODULUS;
        pow /= 2;
    }
    result
}

fn brute_force_attack(base: usize, result: usize) -> usize {
    // Calculate pow such that base^pow = result (mod MODULUS).
    // Uses a simple brute force search.
    assert!(base > 0 && result > 0 && result < MODULUS); // preconditions
    let mut pow = 0;
    let mut prod = 1;
    while prod != result {
        prod = (prod * base) % MODULUS;
        pow += 1;
    }
    pow
}

fn main() {
    let input: Vec<usize> = file_to_vec_parsed("input/day25.txt");
    let (&device_pub, &door_pub) = iter_to_pair(input.iter());
    println!("Device public key: {}", device_pub);
    println!("Door public key: {}", door_pub);

    println!("===== Part 1 =====");
    let starting_base = 7;
    let device_pow = brute_force_attack(starting_base, device_pub);
    let door_pow = brute_force_attack(starting_base, door_pub);
    println!("Device loop size: {}", device_pow);
    println!("Door loop size: {}", door_pow);
    let answer1 = encrypt(starting_base, device_pow * door_pow);
    let answer2 = encrypt(device_pub, door_pow);
    let answer3 = encrypt(door_pub, device_pow);
    assert_eq!(answer1, answer2);
    assert_eq!(answer1, answer3);
    println!("Answer (encryption key): {}", answer1);

    println!("===== Part 2 =====");
    println!("Freebie!");
}
