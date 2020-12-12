/*
    Advent of Code 2020
    Caleb Stanford
    Day 12 Solution
    2020-12-12
*/

use aoc2020::util::file_to_vec;

#[derive(Clone, Copy)]
struct Dir {
    dx: isize,
    dy: isize,
}
const DIR_N: Dir = Dir { dx: 0, dy: 1 };
const DIR_E: Dir = Dir { dx: 1, dy: 0 };
const DIR_S: Dir = Dir { dx: 0, dy: -1 };
const DIR_W: Dir = Dir { dx: -1, dy: 0 };
impl Dir {
    fn turn_clockwise(&mut self) {
        let tmp_dx = self.dx;
        self.dx = self.dy;
        self.dy = -tmp_dx;
    }
}

struct ShipNav {
    waypoint: Dir,
    x: isize,
    y: isize,
}
impl ShipNav {
    fn new() -> Self {
        Self { waypoint: DIR_E, x: 0, y: 0 }
    }
    fn set_waypoint(&mut self, dx: isize, dy: isize) {
        self.waypoint = Dir { dx, dy };
    }
    fn move_ship(&mut self, dir: Dir, amount: isize) {
        debug_assert!(amount > 0);
        self.x += dir.dx * amount;
        self.y += dir.dy * amount;
    }
    fn move_waypoint(&mut self, dir: Dir, amount: isize) {
        debug_assert!(amount > 0);
        self.waypoint.dx += dir.dx * amount;
        self.waypoint.dy += dir.dy * amount;
    }
    fn rotate_waypoint_clockwise(&mut self, amount: isize) {
        debug_assert!(amount % 90 == 0);
        let turns = amount / 90;
        for _i in 0..turns {
            self.waypoint.turn_clockwise()
        }
    }
    fn action_part1(&mut self, action: char, amount: isize) {
        debug_assert!(amount > 0);
        match action {
            'N' => self.move_ship(DIR_N, amount),
            'E' => self.move_ship(DIR_E, amount),
            'S' => self.move_ship(DIR_S, amount),
            'W' => self.move_ship(DIR_W, amount),
            'F' => self.move_ship(self.waypoint, amount),
            'R' => self.rotate_waypoint_clockwise(amount),
            'L' => self.rotate_waypoint_clockwise(360 - amount),
            _ => panic!(),
        }
    }
    fn action_part2(&mut self, action: char, amount: isize) {
        debug_assert!(amount > 0);
        match action {
            'N' => self.move_waypoint(DIR_N, amount),
            'E' => self.move_waypoint(DIR_E, amount),
            'S' => self.move_waypoint(DIR_S, amount),
            'W' => self.move_waypoint(DIR_W, amount),
            'F' => self.move_ship(self.waypoint, amount),
            'R' => self.rotate_waypoint_clockwise(amount),
            'L' => self.rotate_waypoint_clockwise(360 - amount),
            _ => panic!(),
        }
    }
    fn manhattan(&self) -> usize {
        (self.x.abs() + self.y.abs()) as usize
    }
}

fn solve_part1(input: &[(char, isize)]) -> usize {
    let mut ship = ShipNav::new();
    for &(ch, amt) in input {
        ship.action_part1(ch, amt);
    }
    ship.manhattan()
}

fn solve_part2(input: &[(char, isize)]) -> usize {
    let mut ship = ShipNav::new();
    ship.set_waypoint(10, 1);
    for &(ch, amt) in input {
        ship.action_part2(ch, amt);
    }
    ship.manhattan()
}

#[test]
fn test_part1() {
    let input = &[('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
    assert_eq!(solve_part1(input), 25);
}

#[test]
fn test_part2() {
    let input = &[('F', 10), ('N', 3), ('F', 7), ('R', 90), ('F', 11)];
    assert_eq!(solve_part2(input), 286);
}

fn main() {
    let input: Vec<(char, isize)> = file_to_vec("input/day12.txt")
        .iter()
        .map(|s| (s[0..1].parse().unwrap(), s[1..].parse().unwrap()))
        .collect();

    println!("Part 1 Answer: {}", solve_part1(&input));
    println!("Part 2 Answer: {}", solve_part2(&input));
}
