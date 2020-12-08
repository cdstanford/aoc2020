/*
    Advent of Code 2020
    Caleb Stanford
    Utilities
*/

use std::fs::File;
use std::io::{BufRead, BufReader};

// Convert a file to a vector of its lines
pub fn file_to_vec(filepath: &str) -> Vec<String> {
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|maybe_line| maybe_line.unwrap()).collect()
}

// Version that is terminated with an empty line ("")
pub fn file_to_vec_el(filepath: &str) -> Vec<String> {
    let mut v = file_to_vec(filepath);
    v.push("".to_owned());
    v
}

// Separate a line into whitespace-divided parts
pub fn line_to_words(line: &str) -> Vec<String> {
    line.split_whitespace().map(|s| s.to_string()).collect()
}
