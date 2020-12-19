/*
    Advent of Code 2020
    Caleb Stanford
    Day 18 Solution
    2020-12-18

    Start time: 6:11pm
    Solved part 1: 7:21pm
    Solved part 2:
    Code cleanup:

    Time (--release):
*/

use aoc2020::util::file_to_vec_parsed;
use std::convert::TryFrom;
use std::str::FromStr;

/*
    Syntax:
        Binary operations
        Tokens (operations, parens, or numbers)
        Expressions (parsable from strings)

    Expressions implement the desired evaluation logic using a stack.
*/
#[derive(Clone, Copy, Debug)]
enum BinOp {
    Plus,
    Times,
}
impl BinOp {
    fn apply(&self, left: usize, right: usize) -> usize {
        match self {
            BinOp::Plus => left + right,
            BinOp::Times => left * right,
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Token {
    LParen,
    RParen,
    Op(BinOp),
    Num(usize),
}
impl TryFrom<char> for Token {
    type Error = String;
    fn try_from(ch: char) -> Result<Self, String> {
        match ch {
            '(' => Ok(Token::LParen),
            ')' => Ok(Token::RParen),
            '+' => Ok(Token::Op(BinOp::Plus)),
            '*' => Ok(Token::Op(BinOp::Times)),
            _ => match ch.to_digit(10) {
                Some(d) => Ok(Token::Num(d as usize)),
                None => Err(format!("Symbol not recognized: {}", ch)),
            },
        }
    }
}

#[derive(Debug)]
struct Expression {
    tokens: Vec<Token>,
}
impl FromStr for Expression {
    type Err = String;
    fn from_str(raw: &str) -> Result<Self, String> {
        let tokens: Result<Vec<Token>, String> = raw
            .chars()
            .filter(|ch| !ch.is_whitespace())
            .map(Token::try_from)
            .collect();
        Ok(Self { tokens: tokens? })
    }
}
impl Expression {
    fn eval(&self) -> usize {
        // Parsing state consists of a value so far and maybe a pending operation.
        // We start with an implicit "0 +" to simplify things.
        let mut value = 0;
        let mut pending = Some(BinOp::Plus);
        let mut stack: Vec<(usize, BinOp)> = Vec::new();
        for &token in &self.tokens {
            match token {
                Token::LParen => {
                    assert!(pending.is_some());
                    stack.push((value, pending.unwrap()));
                    value = 0;
                    pending = Some(BinOp::Plus);
                }
                Token::RParen => {
                    assert!(pending.is_none());
                    assert!(!stack.is_empty());
                    let (prev_value, prev_pending) = stack.pop().unwrap();
                    value = prev_pending.apply(prev_value, value);
                }
                Token::Op(op) => {
                    assert!(pending.is_none());
                    pending = Some(op);
                }
                Token::Num(n) => {
                    assert!(pending.is_some());
                    value = pending.unwrap().apply(value, n);
                    pending = None;
                }
            }
        }
        assert!(pending.is_none());
        assert!(stack.is_empty());
        value
    }
}

fn solve_part1(input: &[Expression]) -> usize {
    input.iter().map(|e| e.eval()).sum()
}

fn solve_part2(_input: &[Expression]) -> usize {
    0
}

fn main() {
    let input: Vec<Expression> = file_to_vec_parsed("input/day18.txt");

    println!("Solve Part 1: {}", solve_part1(&input));
    println!("Solve Part 2: {}", solve_part2(&input));
}
