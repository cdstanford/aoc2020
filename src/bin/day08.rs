/*
    Advent of Code 2020
    Caleb Stanford
    Day 8 Solution
    2020-12-08
*/

use aoc2020::util::file_to_vec;
use std::collections::HashSet;
use std::fmt;
use std::str::FromStr;

/* Struct for program instructions */

#[derive(Clone, Debug, Eq, PartialEq)]
enum Instruction {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}
impl FromStr for Instruction {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();
        if parts.len() != 2 {
            return Err(format!(
                "Could not parse as instruction, more than 2 parts: {:?}",
                parts,
            ));
        }
        let op: &str = parts[0];
        let arg: isize = parts[1].parse().or_else(|err| {
            Err(format!(
                "Could not parse instruction argument as isize: {} ({})",
                parts[1], err,
            ))
        })?;
        match op {
            "acc" => Ok(Self::Acc(arg)),
            "jmp" => Ok(Self::Jmp(arg)),
            "nop" => Ok(Self::Nop(arg)),
            _ => Err(format!("Could not parse instruction name: {}", op)),
        }
    }
}

/* Struct for program state (how to execute programs) */

type Program = Vec<Instruction>;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Status {
    Running,
    LoopDetected,
    HaltTop,    // program counter goes before the beginning
    HaltBottom, // program counter goes past the end
}
#[derive(Clone, Debug)]
struct State {
    prog: Program,
    prog_counter: isize,
    acc: isize,
    seen: HashSet<isize>,
    status: Status,
}
impl State {
    fn new(prog: Program) -> Self {
        State {
            prog,
            prog_counter: 0,
            acc: 0,
            seen: HashSet::new(),
            status: Status::Running,
        }
    }
    fn is_running(&self) -> bool {
        self.status == Status::Running
    }
    fn step(&mut self) {
        // if not is_running then this will be a no-op
        if self.seen.contains(&self.prog_counter) {
            self.status = Status::LoopDetected;
        } else if self.prog_counter < 0 {
            self.status = Status::HaltTop;
        } else if self.prog_counter as usize >= self.prog.len() {
            self.status = Status::HaltBottom;
        } else {
            self.seen.insert(self.prog_counter);
            let ins = &self.prog[self.prog_counter as usize];
            match ins {
                Instruction::Acc(x) => {
                    self.acc += x;
                    self.prog_counter += 1;
                }
                Instruction::Jmp(x) => {
                    self.prog_counter += x;
                }
                Instruction::Nop(_x) => {
                    self.prog_counter += 1;
                }
            }
        }
    }
    fn execute(&mut self) {
        // Run until we halt or detect a loop
        while self.is_running() {
            // println!("{}", self);
            self.step();
        }
        // println!("{}", self);
    }
}
impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "State: {{pos: {}, acc: {}, status: {:?}}})",
            self.prog_counter, self.acc, self.status,
        )
    }
}

fn solve_part1(program: Program) -> isize {
    let mut st = State::new(program);
    st.execute();
    st.acc
}

fn solve_part2(program: Program) -> isize {
    let mut halt_normally = Vec::new();
    for i in 0..program.len() {
        let mut prog_fixed = program.clone();
        match prog_fixed[i] {
            Instruction::Acc(_x) => {
                continue;
            }
            Instruction::Jmp(x) => {
                prog_fixed[i] = Instruction::Nop(x);
            }
            Instruction::Nop(x) => {
                prog_fixed[i] = Instruction::Jmp(x);
            }
        }
        let mut st = State::new(prog_fixed);
        st.execute();
        match st.status {
            Status::LoopDetected => (),
            Status::HaltBottom => {
                halt_normally.push(st);
            }
            _ => unreachable!(),
        }
    }
    assert_eq!(halt_normally.len(), 1);
    halt_normally[0].acc
}

fn main() {
    let lines = file_to_vec("input/day8.txt");
    let program: Program = lines.iter().map(|s| s.parse().unwrap()).collect();
    // println!("Program: {:?}", program);
    println!("Part 1 Answer: {:?}", solve_part1(program.clone()));
    println!("Part 2 Answer: {:?}", solve_part2(program));
}
