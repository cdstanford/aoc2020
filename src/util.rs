/*
    Advent of Code 2020
    Caleb Stanford
    Utilities
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn file_to_vec(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|maybe_line| maybe_line.unwrap()).collect()
}
