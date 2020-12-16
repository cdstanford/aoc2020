/*
    Advent of Code 2020
    Caleb Stanford
    Utilities
*/

use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

// Convert a file to a vector of its lines
pub fn file_to_vec_parsed<T>(filepath: &str) -> Vec<T>
where
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    let file = File::open(filepath).unwrap();
    let reader = BufReader::new(file);
    reader.lines().map(|line| line.unwrap().parse().unwrap()).collect()
}

// Simple string version
pub fn file_to_vec(filepath: &str) -> Vec<String> {
    file_to_vec_parsed(filepath)
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

// Parse an iterator (e.g. result of split) of length 2 into a tuple
pub fn iter_to_pair<T, I>(elems: I) -> (T, T)
where
    T: Copy,
    I: Iterator<Item = T>,
{
    let v: Vec<T> = elems.collect();
    assert_eq!(v.len(), 2);
    (v[0], v[1])
}
