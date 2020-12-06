/*
    Advent of Code 2020
    Caleb Stanford
    Day 2 Solution
    2020-12-05
*/

use ascii::{AsAsciiStr, AsciiStr, AsciiString};
use regex::Regex;
use std::fs::File;
use std::io::*;

type PasswordInfo = (usize, usize, char, AsciiString);

fn parse_input_line(line: &str) -> PasswordInfo {
    // Note: this compiles a regex multiple times, not optimal.
    // Use lazy_static for better performance.
    let re = Regex::new(r"^(\d+)-(\d+) ([a-z]): ([a-z]*)$").unwrap();
    // Extract capture groups
    let mat = re.captures(&line).unwrap();
    let lb: usize = mat.get(1).unwrap().as_str().parse().unwrap();
    let ub: usize = mat.get(2).unwrap().as_str().parse().unwrap();
    let ch: char = mat.get(3).unwrap().as_str().parse().unwrap();
    let pass: AsciiString = mat
        .get(4)
        .unwrap()
        .as_str()
        .as_ascii_str()
        .unwrap()
        .to_owned();
    (lb, ub, ch, pass)
}

fn count_char_occurences(c: char, s: &AsciiStr) -> usize {
    // count occurences of char in string
    s.chars().filter(|&ch| ch == c).count()
}

fn solve_part1(data: &[PasswordInfo]) -> usize {
    data.iter()
        .filter(|&dat| {
            let pass: &AsciiStr = &dat.3;
            let occurences = count_char_occurences(dat.2, pass);
            dat.0 <= occurences && occurences <= dat.1
        })
        .count()
}

fn solve_part2(data: &[PasswordInfo]) -> usize {
    data.iter()
        .filter(|&dat| {
            let pass: &AsciiStr = &dat.3;
            let i1: usize = dat.0 - 1;
            let i2: usize = dat.1 - 1;
            (pass[i1] == dat.2) ^ (pass[i2] == dat.2)
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ascii(raw: &str) -> AsciiString {
        AsciiString::from_ascii(raw).unwrap()
    }

    fn example_data() -> Vec<PasswordInfo> {
        vec![
            (1, 3, 'a', ascii("abcde")),
            (1, 3, 'b', ascii("cdefg")),
            (2, 9, 'c', ascii("ccccccccc")),
        ]
    }

    #[test]
    fn test_part_1() {
        assert_eq!(solve_part1(&example_data()), 2);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(solve_part2(&example_data()), 1);
    }
}

fn main() {
    let file = File::open("input/day2.txt").unwrap();
    // let file = File::open("input/day2_test.txt").unwrap();
    let reader = BufReader::new(file);
    let data = reader
        .lines()
        .map(|maybe_line| parse_input_line(&maybe_line.unwrap()))
        .collect::<Vec<PasswordInfo>>();
    println!("Part 1 Answer: {}", solve_part1(&data));
    println!("Part 2 Answer: {}", solve_part2(&data));
}
