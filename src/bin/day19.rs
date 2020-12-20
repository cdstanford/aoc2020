/*
    Advent of Code 2020
    Caleb Stanford
    Day 19 Solution
    2020-12-19

    Time (--release):
*/

use aoc2020::util::file_to_vec;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/*
    Specialized matcher designed for matching small strings against a set of
    inter-defined regexes: each can be a union or a concat of two other regexes
    in the set.
    Relies on heavy caching of match results.
*/
type RegexId = u16;
#[derive(Clone, Copy, Debug)]
enum RegexCases {
    Union(RegexId, RegexId),
    Concat(RegexId, RegexId),
    Noop(RegexId),
    Char(char),
}
#[derive(Default)]
struct SmartRegexMatcher {
    regex_defs: HashMap<RegexId, RegexCases>,
    loops_allowed: bool,
    // State related to the current string to match
    match_cache: HashMap<(RegexId, usize, usize), bool>,
    call_stack: HashSet<(RegexId, usize, usize)>,
    // Debug information
    debug: bool,
    cache_hits: usize,
    cache_misses: usize,
    loops_seen: usize,
}
impl SmartRegexMatcher {
    /* Initialization */
    fn new() -> Self {
        Default::default()
    }
    fn add_regex(&mut self, id: RegexId, re: RegexCases) {
        self.regex_defs.insert(id, re);
    }
    fn allow_loops(&mut self) {
        self.loops_allowed = true;
    }
    fn set_debug(&mut self) {
        self.debug = true;
    }

    // Debug info
    fn reset_debug_info(&mut self) {
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.loops_seen = 0;
    }
    fn print_debug_info(&self) {
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
        println!("Loops seen: {}", self.loops_seen);
        println!("Cache size: {}", self.match_cache.len());
    }

    /*
        Functionality
        get_regex and eval_rec are for internal use.
        The only exposed method is eval, which matches a regex against a string.

        An assumption we make is that there are no multiple-byte chars.
    */
    fn get_regex(&mut self, id: RegexId) -> RegexCases {
        *self.regex_defs.get(&id).unwrap()
    }
    fn eval_rec(&mut self, id: RegexId, s: &str, i: usize, j: usize) -> bool {
        if let Some(&result) = self.match_cache.get(&(id, i, j)) {
            if self.debug {
                self.cache_hits += 1;
            }
            result
        } else if self.call_stack.contains(&(id, i, j)) {
            // Loop found
            if self.debug {
                self.loops_seen += 1;
            }
            false
        } else {
            if self.debug {
                self.cache_misses += 1;
            }
            if self.loops_allowed {
                self.call_stack.insert((id, i, j));
            }
            let result = match self.get_regex(id) {
                RegexCases::Union(id1, id2) => {
                    self.eval_rec(id1, s, i, j) || self.eval_rec(id2, s, i, j)
                }
                RegexCases::Concat(id1, id2) => {
                    let mut result = false;
                    for split_point in 0..s.len() {
                        let (s1, s2) = s.split_at(split_point);
                        let b1 = self.eval_rec(id1, s1, i, i + split_point);
                        let b2 = self.eval_rec(id2, s2, i + split_point, j);
                        if b1 & b2 {
                            result = true;
                            break;
                        }
                    }
                    result
                }
                RegexCases::Noop(id1) => self.eval_rec(id1, s, i, j),
                RegexCases::Char(ch) => s == ch.to_string(),
            };
            if self.loops_allowed {
                self.call_stack.remove(&(id, i, j));
            }
            self.match_cache.insert((id, i, j), result);
            result
        }
    }
    fn eval(&mut self, id: RegexId, s: &str) -> bool {
        if self.debug {
            println!("Matching: {}", s);
            println!("String len: {}", s.len());
        }
        let result = self.eval_rec(id, s, 0, s.len());
        if self.debug {
            println!("Result: {}", result);
            self.print_debug_info();
            self.reset_debug_info();
        }
        // Reset caches and return
        self.match_cache = HashMap::new();
        self.call_stack = HashSet::new();
        result
    }
}

fn parse_input(input_filepath: &str) -> (SmartRegexMatcher, Vec<String>) {
    // Regexes to parse input
    // (Better idea: use a proper parsing library)
    let rule = Regex::new(r"^(\d*): (.*)$").unwrap();
    let rule_noop = Regex::new(r"^(\d*)$").unwrap();
    let rule_union = Regex::new(r"^(\d*) \| (\d*)$").unwrap();
    let rule_concat = Regex::new(r"^(\d*) (\d*)$").unwrap();
    let rule_union_concat =
        Regex::new(r"^(\d*) (\d*) \| (\d*) (\d*)$").unwrap();
    let msg = Regex::new(r"^([ab]*)$").unwrap();

    // Parse input
    let mut matcher = SmartRegexMatcher::new();
    let mut msgs: Vec<String> = Vec::new();
    let mut first_part = true;
    for line in &file_to_vec(input_filepath) {
        if let Some(caps) = rule.captures(line) {
            assert!(first_part);
            assert_eq!(caps.len(), 3);
            let id = caps[1].parse::<RegexId>().unwrap();
            let def = &caps[2];
            if def == r#""a""# {
                matcher.add_regex(id, RegexCases::Char('a'));
            } else if def == r#""b""# {
                matcher.add_regex(id, RegexCases::Char('b'));
            } else if let Some(caps) = rule_noop.captures(def) {
                assert_eq!(caps.len(), 2);
                let id1 = caps[1].parse::<RegexId>().unwrap();
                matcher.add_regex(id, RegexCases::Noop(id1));
            } else if let Some(caps) = rule_union.captures(def) {
                assert_eq!(caps.len(), 3);
                let id1 = caps[1].parse::<RegexId>().unwrap();
                let id2 = caps[2].parse::<RegexId>().unwrap();
                matcher.add_regex(id, RegexCases::Union(id1, id2));
            } else if let Some(caps) = rule_concat.captures(def) {
                assert_eq!(caps.len(), 3);
                let id1 = caps[1].parse::<RegexId>().unwrap();
                let id2 = caps[2].parse::<RegexId>().unwrap();
                matcher.add_regex(id, RegexCases::Concat(id1, id2));
            } else if let Some(caps) = rule_union_concat.captures(def) {
                // In this case we generate two fresh IDs
                // Assumption: input IDs are between 0 and 200
                let fresh1 = id + 200;
                let fresh2 = id + 400;
                assert_eq!(caps.len(), 5);
                let id1 = caps[1].parse::<RegexId>().unwrap();
                let id2 = caps[2].parse::<RegexId>().unwrap();
                let id3 = caps[3].parse::<RegexId>().unwrap();
                let id4 = caps[4].parse::<RegexId>().unwrap();
                matcher.add_regex(id, RegexCases::Union(fresh1, fresh2));
                matcher.add_regex(fresh1, RegexCases::Concat(id1, id2));
                matcher.add_regex(fresh2, RegexCases::Concat(id3, id4));
            } else {
                panic!("Parsing error: could not parse rule: {}", def);
            }
        } else if line == "" {
            assert!(first_part);
            first_part = false;
        } else if let Some(caps) = msg.captures(line) {
            assert!(!first_part);
            assert_eq!(caps.len(), 2);
            assert_eq!(line, &caps[0]);
            assert_eq!(line, &caps[1]);
            msgs.push(line.to_string());
        } else {
            panic!("Parsing error: not a rule or msg: {}", line);
        }
    }

    // println!("Rules: {:?}", matcher.regex_defs);
    // println!("Messages: {:?}", msgs);

    (matcher, msgs)
}

fn solve_part1() -> usize {
    let (mut matcher, msgs) = parse_input("input/day19.txt");
    matcher.set_debug();
    msgs.iter().map(|s| matcher.eval(0, s)).filter(|&s| s).count()
}

fn solve_part2() -> usize {
    let (mut matcher, _msgs) = parse_input("input/day19.txt");
    // Additional rules:
    //     8: 42 | 42 8
    //     11: 42 31 | 42 11 31
    matcher.allow_loops();
    matcher.add_regex(8, RegexCases::Union(42, 208));
    matcher.add_regex(208, RegexCases::Union(42, 8));
    matcher.add_regex(11, RegexCases::Union(211, 411));
    matcher.add_regex(211, RegexCases::Union(42, 31));
    matcher.add_regex(411, RegexCases::Union(42, 611));
    matcher.add_regex(611, RegexCases::Union(11, 31));

    matcher.set_debug();
    // Not working yet
    // msgs.iter().map(|s| matcher.eval(0, s)).filter(|&s| s).count()
    0
}

fn main() {
    println!("Part 1 Answer: {}", solve_part1());
    println!("Part 2 Answer: {}", solve_part2());
}
