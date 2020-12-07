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

// version that is terminated with an empty line ("")
pub fn file_to_vec_el(filepath: &str) -> Vec<String> {
    let mut v = file_to_vec(filepath);
    v.push("".to_owned());
    v
}
