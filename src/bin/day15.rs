/*
    Advent of Code 2020
    Caleb Stanford
    Day 15 Solution
    2020-12-15

    Start time: 11:42am
    Solved part 1: 12:11pm (29 min)
    Solved part 2: 12:12pm (1 min)
    Code cleanup: 12:23pm

    Time (--release): 0m3.466s
*/

use aoc2020::util::file_to_vec_parsed;
use std::collections::HashMap;

#[derive(Default)]
struct GameState {
    // Most recent turn number and number spoken (None if no turns yet)
    turn: usize,
    last_spoken: Option<usize>,
    // Distance when last_spoken was previously said, if it was a repeat
    distance: Option<usize>,
    // For each spoken number, the most recent turn it was said
    memory: HashMap<usize, usize>,
}
impl GameState {
    // Initial value and final answer
    fn new() -> Self {
        Default::default()
    }
    fn get_last_spoken(&self) -> usize {
        assert!(self.turn > 0);
        self.last_spoken.unwrap()
    }
    // Starting turns call speak.
    // Turns after that call memory_turn.
    fn speak(&mut self, num: usize) {
        self.turn += 1;
        self.last_spoken = Some(num);
        self.distance = match self.memory.get(&num) {
            None => None,
            Some(prev) => Some(self.turn - prev),
        };
        self.memory.insert(num, self.turn);
    }
    fn memory_turn(&mut self) {
        self.speak(self.distance.unwrap_or(0));
    }
}

fn solve_game(start_nums: &[usize], turns: usize) -> usize {
    let mut game = GameState::new();
    for i in 0..turns {
        if i < start_nums.len() {
            game.speak(start_nums[i]);
        } else {
            game.memory_turn();
        }
    }
    game.get_last_spoken()
}

fn main() {
    let start_nums: Vec<usize> = file_to_vec_parsed("input/day15.txt");

    println!("Part 1 Answer: {}", solve_game(&start_nums, 2020));
    println!("Part 2 Answer: {}", solve_game(&start_nums, 30000000));
}
