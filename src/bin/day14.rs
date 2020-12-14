/*
    Advent of Code 2020
    Caleb Stanford
    Day 14 Solution
    2020-12-14
*/

use aoc2020::util::{file_to_vec, line_to_words};
use std::collections::HashMap;

fn parse_binary(raw: &str) -> u64 {
    u64::from_str_radix(raw, 2).unwrap()
}

/*
    Bit Mask Logic
    The mask is stored as two unsigned integers, where X = 0 and X = 1
    respectively.
*/
type Mask = (u64, u64);
fn parse_mask(raw: &str) -> Mask {
    (parse_binary(&raw.replace("X", "0")), parse_binary(&raw.replace("X", "1")))
}
fn parse_all_masks(raw: &str) -> Vec<Mask> {
    // For part 2: parse all possible masks.
    // 0 becomes X, 1 becomes 1, and X becomes either 0 or 1.
    let mut results = vec!["".to_owned()];
    for ch in raw.chars() {
        let to_append = match ch {
            '0' => vec!['X'],
            '1' => vec!['1'],
            'X' => vec!['0', '1'],
            _ => unreachable!(),
        };
        let mut new_results = Vec::new();
        for prev in &results {
            for new_ch in &to_append {
                new_results.push(prev.to_owned() + &new_ch.to_string());
            }
        }
        results = new_results;
    }
    results.iter().map(|s| parse_mask(s)).collect()
}
fn apply_mask(m: Mask, n: u64) -> u64 {
    m.0 | (m.1 & n)
}

/*
    Available commands
    (and how they are executed)
*/
enum Command {
    SetMask(String),
    SetMem(u64, u64),
}
fn parse_command(raw: &str) -> Command {
    let words = line_to_words(raw);
    assert_eq!(words.len(), 3);
    assert_eq!(words[1], "=");
    if words[0] == "mask" {
        Command::SetMask(words[2].to_owned())
    } else {
        let w0len = words[0].len();
        let loc = words[0].get(4..(w0len - 1)).unwrap().parse().unwrap();
        let val = words[2].parse().unwrap();
        Command::SetMem(loc, val)
    }
}
struct ProgState {
    mask: String,
    memory: HashMap<u64, u64>,
}
impl ProgState {
    fn new() -> Self {
        ProgState { mask: "X".to_owned(), memory: HashMap::new() }
    }
    fn execute_part1(&mut self, command: &Command) {
        match command {
            Command::SetMask(m) => {
                self.mask = m.to_owned();
            }
            &Command::SetMem(loc, val) => {
                let masked_val = apply_mask(parse_mask(&self.mask), val);
                self.memory.insert(loc, masked_val);
            }
        }
    }
    fn execute_part2(&mut self, command: &Command) {
        match command {
            Command::SetMask(m) => {
                self.mask = m.to_owned();
            }
            &Command::SetMem(loc, val) => {
                let masks = parse_all_masks(&self.mask);
                for &mask in &masks {
                    let masked_loc = apply_mask(mask, loc);
                    self.memory.insert(masked_loc, val);
                }
            }
        }
    }
}

fn solve_part1(prog: &[Command]) -> u64 {
    let mut state = ProgState::new();
    for comm in prog {
        state.execute_part1(comm);
    }
    state.memory.iter().map(|(&_k, &v)| v).sum()
}

fn solve_part2(prog: &[Command]) -> u64 {
    let mut state = ProgState::new();
    for comm in prog {
        state.execute_part2(comm);
    }
    state.memory.iter().map(|(&_k, &v)| v).sum()
}

fn main() {
    let commands: Vec<Command> = file_to_vec("input/day14.txt")
        .iter()
        .map(|s| parse_command(s))
        .collect();

    println!("Part 1 Answer: {}", solve_part1(&commands));
    println!("Part 2 Answer: {}", solve_part2(&commands));
}
