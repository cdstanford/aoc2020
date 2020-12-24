/*
    Advent of Code 2020
    Caleb Stanford
    Utilities
*/

use std::collections::HashSet;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

/* Parsing */

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
pub fn iter_to_pair<T, I>(mut elems: I) -> (T, T)
where
    I: Iterator<Item = T>,
    T: Debug + PartialEq,
{
    let elem1 = elems.next().unwrap();
    let elem2 = elems.next().unwrap();
    assert_eq!(elems.next(), None);
    (elem1, elem2)
}

/* Useful iterators */

pub fn iter_prod<T, IterT, U, IterU>(
    iter_t: IterT,
    iter_u: IterU,
) -> impl Iterator<Item = (T, U)>
where
    T: Clone,
    IterT: Iterator<Item = T>,
    IterU: Iterator<Item = U> + Clone,
{
    iter_t.flat_map(move |t| iter_u.clone().map(move |u| (t.clone(), u)))
}

pub fn iter_rectangle(
    x0: isize,
    y0: isize,
    x1: isize,
    y1: isize,
) -> impl Iterator<Item = (isize, isize)> {
    iter_prod(x0..=x1, y0..=y1)
}

/* Validation */

// Check if a list of integers contains every number from 1 to n, for some n.
pub fn unique_1_to_n<'a, I: Iterator<Item = &'a usize>>(ints: I) -> bool {
    let mut seen = HashSet::new();
    let mut high = None;
    for &i in ints {
        if i == 0 || seen.contains(&i) {
            return false;
        }
        seen.insert(i);
        high = high.max(Some(i));
    }
    high.unwrap_or(0) == seen.len()
}

// Version that checks between 0 and n instead
pub fn unique_0_to_n<'a, I: Iterator<Item = &'a usize>>(ints: I) -> bool {
    let mut seen = HashSet::new();
    let mut high = None;
    for &i in ints {
        if seen.contains(&i) {
            return false;
        }
        seen.insert(i);
        high = high.max(Some(i));
    }
    high.map_or(0, |x| x + 1) == seen.len()
}

// Weaker version that only checks uniqueness
pub fn unique<'a, I: Iterator<Item = &'a usize>>(ints: I) -> bool {
    let mut seen = HashSet::new();
    for &i in ints {
        if i == 0 || seen.contains(&i) {
            return false;
        }
        seen.insert(i);
    }
    true
}

/* Unit tests */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unique_1_to_n() {
        assert!(unique_1_to_n([].iter()));
        assert!(unique_1_to_n([1].iter()));
        assert!(unique_1_to_n([1, 2].iter()));
        assert!(unique_1_to_n([2, 1].iter()));
        assert!(unique_1_to_n([1, 2, 3, 4, 5].iter()));
        assert!(unique_1_to_n([5, 2, 4, 1, 3].iter()));
        assert!(unique_1_to_n([1, 2, 5, 4, 3].iter()));
        assert!(!unique_1_to_n([0].iter()));
        assert!(!unique_1_to_n([2].iter()));
        assert!(!unique_1_to_n([1, 1].iter()));
        assert!(!unique_1_to_n([1, 3].iter()));
        assert!(!unique_1_to_n([3, 2].iter()));
        assert!(!unique_1_to_n([5, 5].iter()));
        assert!(!unique_1_to_n([1, 2, 0].iter()));
        assert!(!unique_1_to_n([1, 2, 4, 4, 5].iter()));
        assert!(!unique_1_to_n([1, 2, 3, 4, 6].iter()));
    }
}
