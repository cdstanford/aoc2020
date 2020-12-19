/*
    Advent of Code 2020
    Caleb Stanford
    Day 18 Solution
    2020-12-18

    Start time: 6:11pm
    Solved part 1: 7:21pm (1 hr, 10 min)
    Solved part 2: 8:31pm (1 hr, 10 min)
    Code cleanup:

    Time (--release): 0m0.051s
*/

use aoc2020::util::file_to_vec_parsed;
use std::convert::TryFrom;
use std::str::FromStr;

/*
    I solved this by defining expressions to be sequences of tokens
    (including parentheses) and evaluating left-to-right using a stack.
    Using this method it is tricky to think of the correct state to store
    during evaluation. There should be a more elegant solution, perhaps
    by extracting the parse tree before evaluation or by using libraries which
    are dedicated to parsing.

    The following data types define the syntax:
        Binary operations (plus and times)
        Tokens (operations, parens, or numbers)
        Expressions (parsable from strings)
*/
#[derive(Clone, Copy, Debug, PartialEq)]
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
    fn eval_part1(&self) -> usize {
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
    fn eval_part2(&self) -> usize {
        // Parsing state consists of a product so far, a sum so far (in the
        // latest group), and whether or not an operation is pending.
        // We don't need to know whether the operation is + or * because
        // we implicitly interpret * as "* 0 +". Similarly, our initial
        // state will have product 1, and sum so far 0 with a pending +.
        let mut state_prod = 1;
        let mut state_sum = 0;
        let mut pending = true;
        let mut stack: Vec<(usize, usize)> = Vec::new();
        for &token in &self.tokens {
            // println!(
            //     ">> state: {:?} {} {} {}",
            //     stack, state_prod, state_sum, pending
            // );
            // println!(">> reading: {:?}", token);
            match token {
                Token::LParen => {
                    assert!(pending);
                    stack.push((state_prod, state_sum));
                    state_prod = 1;
                    state_sum = 0;
                    pending = true;
                }
                Token::RParen => {
                    assert!(!pending);
                    assert!(!stack.is_empty());
                    let (prev_prod, prev_sum) = stack.pop().unwrap();
                    state_sum = prev_sum + (state_prod * state_sum);
                    state_prod = prev_prod;
                }
                Token::Op(op) => {
                    assert!(!pending);
                    if op == BinOp::Times {
                        state_prod *= state_sum;
                        state_sum = 0;
                    }
                    pending = true;
                }
                Token::Num(n) => {
                    assert!(pending);
                    state_sum += n;
                    pending = false;
                }
            }
        }
        // println!(
        //     ">> final state: {:?} {} {} {}",
        //     stack, state_prod, state_sum, pending
        // );
        assert!(!pending);
        assert!(stack.is_empty());
        // println!("Result: {}", state_prod * state_sum);
        state_prod * state_sum
    }
}

fn solve_part1(input: &[Expression]) -> usize {
    input.iter().map(|e| e.eval_part1()).sum()
}

#[cfg(test)]
fn assert_part2(raw: &str, expected: usize) {
    assert_eq!(Expression::from_str(raw).unwrap().eval_part2(), expected);
}
#[test]
fn test_part2() {
    assert_part2("2 + 3", 5);
    assert_part2("2 * 3", 6);
    assert_part2("2 + 2 + 3", 7);
    assert_part2("2 + 2 * 3", 12);
    assert_part2("2 * 3 + 2", 10);
    assert_part2("6 * (2 + 2)", 24);
}

fn solve_part2(input: &[Expression]) -> usize {
    input.iter().map(|e| e.eval_part2()).sum()
}

fn main() {
    let input: Vec<Expression> = file_to_vec_parsed("input/day18.txt");

    println!("Solve Part 1: {}", solve_part1(&input));
    println!("Solve Part 2: {}", solve_part2(&input));
}
