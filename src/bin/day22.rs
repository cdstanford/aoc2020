/*
    Advent of Code 2020
    Caleb Stanford
    Day 22 Solution
    2020-12-22

    Time (--release): 0m0.506s
    Time (--debug): 0m9.072s
*/

use aoc2020::util::{file_to_vec, iter_to_pair};
use std::cmp::Ordering;
use std::collections::hash_map::DefaultHasher;
use std::collections::{HashSet, VecDeque};
use std::hash::{Hash, Hasher};

// A fun utility function to check if a list of integers contains every
// number from 1 to n, for some n.
fn unique_1_to_n<'a, I: Iterator<Item = &'a usize>>(ints: I) -> bool {
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
// Weaker version for recursive games in part 2: only checks uniqueness
fn unique<'a, I: Iterator<Item = &'a usize>>(ints: I) -> bool {
    let mut seen = HashSet::new();
    for &i in ints {
        if i == 0 || seen.contains(&i) {
            return false;
        }
        seen.insert(i);
    }
    true
}

/*
    Basic types

    Card is a simple wrapper around usize.
    However, we deliberately do not derive Copy or Clone.
    This has the nice guarantee that we know cards won't be duplicated during
    the game, which matches the reality of physical cards and ensures we
    don't make a mistake like pushing a card onto both player's decks.
*/
#[derive(Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Card(usize);
#[derive(Clone, Copy, Debug)]
enum Player {
    One,
    Two,
}

/*
    The SpaceCards game state, implementing both part 1 and part 2 logic.
*/
#[derive(Debug)]
struct SpaceCards {
    deck1: VecDeque<Card>,
    deck2: VecDeque<Card>,
    winner: Option<Player>,
    seen_hashes: HashSet<u64>,
}
impl SpaceCards {
    /*
        Constructor for a new game.
        In debug mode, this validates that the deck cards provided are unique,
        and optionally further that they are equal to 1 through n for some n.
    */
    fn debug_checks(
        start_deck1: &[usize],
        start_deck2: &[usize],
        verify_all_cards_present: bool,
    ) -> bool {
        let all_cards: Vec<usize> =
            start_deck1.iter().chain(start_deck2.iter()).copied().collect();
        if verify_all_cards_present {
            unique_1_to_n(all_cards.iter())
        } else {
            unique(all_cards.iter())
        }
    }
    fn new(
        start_deck1: &[usize],
        start_deck2: &[usize],
        verify_all_cards_present: bool,
    ) -> Self {
        let deck1 = start_deck1.iter().map(|&i| Card(i)).collect();
        let deck2 = start_deck2.iter().map(|&i| Card(i)).collect();
        debug_assert!(Self::debug_checks(
            start_deck1,
            start_deck2,
            verify_all_cards_present
        ));
        let winner = None;
        let seen_hashes = HashSet::new();
        Self { deck1, deck2, winner, seen_hashes }
    }

    /*
        Game score and printing functionality.
    */
    fn deck_score(deck: &VecDeque<Card>) -> usize {
        deck.iter().rev().enumerate().map(|(i, Card(j))| (i + 1) * j).sum()
    }
    fn print_state(&self) {
        print!("Player 1 deck:");
        for &Card(i) in &self.deck1 {
            print!(" {}", i);
        }
        println!();
        print!("Player 2 deck:");
        for &Card(i) in &self.deck2 {
            print!(" {}", i);
        }
        println!();
    }
    fn print_end_state(&self) {
        match self.winner {
            Some(Player::One) => {
                debug_assert!(self.deck2.is_empty());
                println!("Player 1 wins!");
                self.print_state();
                println!("Score (answer): {}", Self::deck_score(&self.deck1));
            }
            Some(Player::Two) => {
                debug_assert!(self.deck1.is_empty());
                println!("Player 2 wins!");
                self.print_state();
                println!("Score (answer): {}", Self::deck_score(&self.deck2));
            }
            None => panic!("End state called on game still in progress!"),
        }
    }

    /*
        Part 1 Rules
    */
    fn part1_step(&mut self) -> bool {
        if self.deck1.is_empty() {
            self.winner = Some(Player::Two);
            false
        } else if self.deck2.is_empty() {
            self.winner = Some(Player::One);
            false
        } else {
            let c1 = self.deck1.pop_front().unwrap();
            let c2 = self.deck2.pop_front().unwrap();
            match c1.cmp(&c2) {
                Ordering::Less => {
                    self.deck2.push_back(c2);
                    self.deck2.push_back(c1);
                }
                Ordering::Greater => {
                    self.deck1.push_back(c1);
                    self.deck1.push_back(c2);
                }
                Ordering::Equal => unreachable!(),
            }
            true
        }
    }
    fn part1_execute(&mut self) {
        while self.part1_step() {}
    }

    /*
        Part 2 Rules

        The one iffy thing we do is store the game state as a u64 hash
        instead of as a truly unique value, which is mainly to avoid dealing
        with a hashset over the entire state (VecDeque<Card>, VecDeque<Card>)
        and a lot of associated copying/cloning.
        Depending on how long typical games are, which I don't know, the
        probability of a collision may be sufficiently low to justify this.
        It at least gives the correct answer on the provided input.
    */
    fn state_hash(&self) -> u64 {
        let mut hasher = DefaultHasher::new();
        self.deck1.hash(&mut hasher);
        self.deck2.hash(&mut hasher);
        hasher.finish()
    }
    fn top_cards(deck: &VecDeque<Card>, n: usize) -> Vec<usize> {
        // Precondition: deck has at least n cards
        let result: Vec<_> = deck.iter().take(n).map(|x| x.0).collect();
        debug_assert_eq!(result.len(), n);
        result
    }
    fn part2_step(&mut self) -> bool {
        // Check for repeated state -- player 1 wins on repetition
        let state = self.state_hash();
        if self.seen_hashes.contains(&state) {
            self.winner = Some(Player::One);
            return false;
        }
        self.seen_hashes.insert(state);
        // Check for deck empty (same as in part 1)
        if self.deck1.is_empty() {
            self.winner = Some(Player::Two);
            return false;
        } else if self.deck2.is_empty() {
            self.winner = Some(Player::One);
            return false;
        }
        // Draw cards
        let c1 = self.deck1.pop_front().unwrap();
        let c2 = self.deck2.pop_front().unwrap();
        let round_winner =
            if self.deck1.len() >= c1.0 && self.deck2.len() >= c2.0 {
                // Recursive combat!!!
                let new_deck1 = Self::top_cards(&self.deck1, c1.0);
                let new_deck2 = Self::top_cards(&self.deck2, c2.0);
                let mut rec_game = Self::new(&new_deck1, &new_deck2, false);
                rec_game.part2_execute()
            } else {
                // Normal rules (same as in part 1)
                match c1.cmp(&c2) {
                    Ordering::Greater => Player::One,
                    Ordering::Less => Player::Two,
                    Ordering::Equal => unreachable!(),
                }
            };
        // Push cards back on deck
        match round_winner {
            Player::One => {
                self.deck1.push_back(c1);
                self.deck1.push_back(c2);
            }
            Player::Two => {
                self.deck2.push_back(c2);
                self.deck2.push_back(c1);
            }
        }
        true
    }
    fn part2_execute(&mut self) -> Player {
        while self.part2_step() {}
        self.winner.unwrap()
    }
}

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

fn parse_input(lines: &[String]) -> (Vec<usize>, Vec<usize>) {
    let (p1_lines, p2_lines) = iter_to_pair(lines.split(|line| line == ""));
    assert_eq!(p1_lines[0], "Player 1:");
    assert_eq!(p2_lines[0], "Player 2:");
    let deck1 =
        p1_lines.iter().skip(1).map(|line| line.parse().unwrap()).collect();
    let deck2 =
        p2_lines.iter().skip(1).map(|line| line.parse().unwrap()).collect();
    (deck1, deck2)
}

fn main() {
    let lines = file_to_vec("input/day22.txt");
    let (starting_deck1, starting_deck2) = parse_input(&lines);
    let mut game1 = SpaceCards::new(&starting_deck1, &starting_deck2, true);
    let mut game2 = SpaceCards::new(&starting_deck1, &starting_deck2, true);

    println!("===== Part 1 =====");
    game1.print_state();
    game1.part1_execute();
    game1.print_end_state();

    println!("===== Part 2 =====");
    game2.print_state();
    game2.part2_execute();
    game2.print_end_state();
}
