/*
    Advent of Code 2020
    Caleb Stanford
    Day 23 Solution
    2020-12-23

    Time (--release): 0m1.979s
*/

use aoc2020::util::{file_to_vec_parsed, unique_0_to_n};
use std::char;
use std::iter;

/*
    Cups are stored and identified using IDs from 0 to 8.
    When printing, we add 1 to get a label from 1 to 9.

    In an alternate formulation we could make n a constant, or use const
    generics to parameterize Cup over n. While making n a field is not really
    a significant overhead (since Cups are not created/deleted during the
    game), it does have the added disadvantage that we have to ensure and
    validate ourselves that all the different ns are the same.
*/
struct Cup {
    n: usize,   // 9
    id: usize,  // 0 to 8
    fwd: usize, // ID of cup in front (clockwise)
    bck: usize, // ID of cup behind (counterclockwise)
}
impl Cup {
    fn display(&self) -> char {
        if self.n > 9 {
            // Functionality not needed for this problem
            unimplemented!()
        } else {
            char::from_digit((self.id + 1) as u32, 10).unwrap()
        }
    }
}
fn wrap_inc(id: usize, n: usize) -> usize {
    (id + 1) % n
}
fn wrap_dec(id: usize, n: usize) -> usize {
    (id + n - 1) % n
}

/*
    The game state is then stored as a vector of 9 cups *in order of ID*.
    This allows O(1) update to the game state, since we don't move the
    cups around, we just update the fwd/bck pointers to other cups.
*/
struct CupGame {
    size: usize,
    curr: usize,
    cups: Vec<Cup>,
}
impl CupGame {
    /* Iterators */
    fn cups_clockwise_from<'a>(
        &'a self,
        start: usize,
    ) -> impl Iterator<Item = usize> + 'a {
        iter::successors(Some(start), move |&i| Some(self.cups[i].fwd))
            .take(self.size)
    }
    fn cups_clockwise<'a>(&'a self) -> impl Iterator<Item = usize> + 'a {
        self.cups_clockwise_from(self.curr)
    }
    fn cups_downward_from(&self, start: usize) -> impl Iterator<Item = usize> {
        // This one doesn't need lifetime annotations since we can save n
        let n = self.size;
        iter::successors(Some(start), move |&i| Some(wrap_dec(i, n))).take(n)
    }

    /* Invariant checker */
    // Returns true so it can be used with assert! and debug_assert!
    fn check_invariant(&self) -> bool {
        assert_eq!(self.cups.len(), self.size);
        assert!(self.curr < self.size);
        for (i, cup) in self.cups.iter().enumerate() {
            assert_eq!(cup.n, self.size);
            assert_eq!(cup.id, i);
            // fwd and bck are inverse functions
            assert_eq!(self.cups[cup.fwd].bck, i);
            assert_eq!(self.cups[cup.bck].fwd, i);
        }
        // fwd is a permutation of 0..(n-1) and moreover an n-cycle
        let clockwise: Vec<usize> = self.cups_clockwise().collect();
        assert_eq!(clockwise.len(), self.size);
        assert!(unique_0_to_n(clockwise.iter()));
        // redundant additional sanity checks
        assert!(unique_0_to_n(self.cups.iter().map(|cup| &cup.fwd)));
        assert!(unique_0_to_n(self.cups.iter().map(|cup| &cup.bck)));
        true
    }

    /* Create a new game */
    // Note that the cups are from 1 to n so we need to subtract 1 everywhere
    fn new(starting_cups: &[usize]) -> Self {
        let size = starting_cups.len();
        let curr = starting_cups[0] - 1;
        let mut cups: Vec<Cup> = Vec::new();
        for i in 0..size {
            cups.push(Cup {
                n: size,
                id: i,
                fwd: size, // placeholder
                bck: size, // placeholder
            });
        }
        for i in 0..size {
            let prev = starting_cups[wrap_dec(i, size)] - 1;
            let this = starting_cups[i] - 1;
            let next = starting_cups[wrap_inc(i, size)] - 1;
            cups[this].fwd = next;
            cups[this].bck = prev;
        }
        let result = CupGame { size, curr, cups };
        assert!(result.check_invariant());
        result
    }

    /* Printing */
    // The .cups_clockwise() iterator makes this really nice!
    fn display(&self) -> String {
        self.cups_clockwise().map(|i| self.cups[i].display()).collect()
    }
    fn display_from(&self, start: usize) -> String {
        self.cups_clockwise_from(start - 1)
            .map(|i| self.cups[i].display())
            .collect()
    }

    /* Game logic */
    fn step(&mut self) {
        // Get cups that need to be moved (cup1, cup2, and cup3), together with
        // the surrounding cups)
        let mut cup_iter = self.cups_clockwise();
        let cup0 = cup_iter.next().unwrap();
        let cup1 = cup_iter.next().unwrap();
        let cup2 = cup_iter.next().unwrap();
        let cup3 = cup_iter.next().unwrap();
        let cup4 = cup_iter.next().unwrap();
        drop(cup_iter);

        // Destination slot
        let dest = self
            .cups_downward_from(self.curr)
            .find(|&i| i != cup0 && i != cup1 && i != cup2 && i != cup3)
            .unwrap();
        let dest_next = self.cups[dest].fwd;

        // Move the cups
        self.cups[cup0].fwd = cup4;
        self.cups[cup4].bck = cup0;
        self.cups[dest].fwd = cup1;
        self.cups[cup1].bck = dest;
        self.cups[cup3].fwd = dest_next;
        self.cups[dest_next].bck = cup3;

        // Update current
        self.curr = cup4;

        // In debug mode, verify we didn't screw anything up
        debug_assert!(self.check_invariant());
    }
    fn step_for(&mut self, iterations: usize) {
        for _ in 0..iterations {
            self.step();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game() {
        let starting = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
        let mut game = CupGame::new(&starting);
        assert_eq!(&game.display(), "389125467");
        game.step();
        assert_eq!(&game.display(), "289154673");
        game.step();
        assert_eq!(&game.display(), "546789132");
        game.step_for(8);
        assert_eq!(&game.display(), "837419265");
    }
}

fn main() {
    let input: Vec<usize> = file_to_vec_parsed("input/day23.txt");

    println!("===== Part 1 =====");
    let mut game = CupGame::new(&input);
    println!("Start state: {}", game.display());
    game.step_for(100);
    println!("End state: {}", game.display());
    println!("Answer: {}", &game.display_from(1)[1..]);

    println!("===== Part 2 =====");
    let mut input = input;
    input.append(&mut (10..=1000000).collect());
    let mut game = CupGame::new(&input);
    game.step_for(10000000);
    let mut iter = game.cups_clockwise_from(0);
    assert_eq!(iter.next().unwrap() + 1, 1);
    let star1 = iter.next().unwrap() + 1;
    let star2 = iter.next().unwrap() + 1;
    println!("Answer: {} x {} = {}", star1, star2, star1 * star2);
}
