/*
    Advent of Code 2020
    Caleb Stanford
    Day 19 Solution
    2020-12-19 to 2020-12-20

    Time (--release): 5m16.695s
*/

use aoc2020::util::file_to_vec;
use regex::Regex;
use std::collections::{HashMap, HashSet};

/*
    SmartRegexMatcher

    Specialized matcher designed for matching small strings against a set of
    inter-defined regexes (rules): each inter-defined regex can be a union or
    a concat of two other regexes in the set.

    This solution is not very efficient (5 minutes whereas there should be
    a solution that works in seconds), but it works.

    The assumption here is that it would be inefficient to simply expand out
    regex 0 into a single regex recursively, as the expression tree might
    contain the same regex many times (this is true even without loops);
    similar to how a Boolean formula can be exponentially larger than a Boolean
    circuit, in the worst case we might get a regex of size 2^m if we started
    from m inter-defined regex rules.

    # Part 1 solution and complexity
    Our idea is to heavily rely on caching of match results.
    For each string we are asked to match, we recursively call match on
    substrings as appropriate, but as we do so we keep a memoization cache
    of the match results for each (regex, start index, end index) triple.
    As a result we are guaranteed to recurse only once for each such tuple,
    which bounds the number of cache misses by O(n^2 m), and since each cache
    miss does O(n) work (for the Concat case, recursing on all O(n) splits),
    the worst-case time complexity is given by
        O(n^3 m),
    where n is the length of the string and m is the number of regexes (rules).

    # Part 2 solution and complexity
    For part 2, to deal with loops, we just need to additionally track (as we
    recurse on regexes and substrings) the call stack (as a set) of which
    (regex, start index, end index) triples we have seen. If we attempt to
    recurse on a triple that is already in the call stack set, we know that
    this is a loop and there is no need to explore it. Basically, each string
    which matches must have a match that does not contain any loops in the match
    tree. Keeping this additional information doesn't add any time overhead
    beyond O(1) for each recursive call (to update the call stack before/after),
    so the complexity is still
        O(n^3 m).

    # Concrete time complexity
    With the worst-case of a string of length 100 and 130 rules, this gives
        130,000,000
    operations per match.

    # Space complexity
    Since we reset the cache after each string match, the memory complexity
    (cache size) is O(n^2 m) for part 1. For part 2, there is no a priori bound
    on the size of the call stack but in practice it seems to be low enough.
*/

type RegexId = u16;
const MAX_ID: RegexId = 200;
fn base_id(id: u16) -> RegexId {
    debug_assert!(id < MAX_ID, "ID {} too large: MAX_ID is {}", id, MAX_ID);
    id
}
fn fresh_id(id: u16, offset: u16) -> RegexId {
    debug_assert!(offset >= 1);
    base_id(id) + offset * MAX_ID
}
fn parse_id(id_str: &str) -> RegexId {
    let id = id_str.parse::<RegexId>().unwrap_or_else(|err| {
        panic!("Could not parse ID (u16): {} ({})", id_str, err)
    });
    base_id(id)
}

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
    #[cfg(debug_assertions)]
    cache_hits: usize,
    #[cfg(debug_assertions)]
    cache_misses: usize,
    #[cfg(debug_assertions)]
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

    /* Debug info */
    #[cfg(debug_assertions)]
    fn reset_debug_info(&mut self) {
        self.cache_hits = 0;
        self.cache_misses = 0;
        self.loops_seen = 0;
    }
    #[cfg(debug_assertions)]
    fn cache_hit(&mut self) {
        self.cache_hits += 1;
    }
    #[cfg(debug_assertions)]
    fn cache_miss(&mut self) {
        self.cache_misses += 1;
    }
    #[cfg(debug_assertions)]
    fn loop_seen(&mut self) {
        self.loops_seen += 1;
    }
    #[cfg(debug_assertions)]
    fn print_debug_info(&self) {
        println!("Cache hits: {}", self.cache_hits);
        println!("Cache misses: {}", self.cache_misses);
        println!("Loops seen: {}", self.loops_seen);
        println!("Cache size: {}", self.match_cache.len());
    }

    #[cfg(not(debug_assertions))]
    fn reset_debug_info(&self) {}
    #[cfg(not(debug_assertions))]
    fn cache_hit(&self) {}
    #[cfg(not(debug_assertions))]
    fn cache_miss(&self) {}
    #[cfg(not(debug_assertions))]
    fn loop_seen(&self) {}
    #[cfg(not(debug_assertions))]
    fn print_debug_info(&self) {}

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
            self.cache_hit();
            result
        } else if self.call_stack.contains(&(id, i, j)) {
            // Loop found
            self.loop_seen();
            false
        } else {
            self.cache_miss();
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
        println!("Matching: {}", s);
        println!("String len: {}", s.len());
        let result = self.eval_rec(id, s, 0, s.len());
        println!("Result: {}", result);
        self.print_debug_info();
        self.reset_debug_info();
        // Reset caches and return
        self.match_cache = HashMap::new();
        self.call_stack = HashSet::new();
        result
    }

    /* Answer */
    fn count_regex0_matches(&mut self, msgs: &[String]) -> usize {
        msgs.iter().map(|s| self.eval(0, s)).filter(|&s| s).count()
    }
}

/*
    Input parsing and parts 1+2 solutions
*/

fn parse_input(input_lines: &[String]) -> (SmartRegexMatcher, Vec<String>) {
    // Regexes to parse input
    // (Better idea: use a proper parsing library)
    let rule = Regex::new(r"^(\d*): (.*)$").unwrap();
    let rule_noop = Regex::new(r"^(\d*)$").unwrap();
    let rule_union = Regex::new(r"^(\d*) \| (\d*)$").unwrap();
    let rule_concat = Regex::new(r"^(\d*) (\d*)$").unwrap();
    let rule_union_concat =
        Regex::new(r"^(\d*) (\d*) \| (\d*) (\d*)$").unwrap();
    let msg = Regex::new(r"^([ab]*)$").unwrap();

    // Collect lines into a SmartRegexMatcher and list of messages
    let mut matcher = SmartRegexMatcher::new();
    let mut msgs: Vec<String> = Vec::new();
    let mut first_part = true;
    for line in input_lines {
        if let Some(caps) = rule.captures(line) {
            assert!(first_part);
            assert_eq!(caps.len(), 3);
            let id = parse_id(&caps[1]);
            let def = &caps[2];
            if def == r#""a""# {
                matcher.add_regex(id, RegexCases::Char('a'));
            } else if def == r#""b""# {
                matcher.add_regex(id, RegexCases::Char('b'));
            } else if let Some(caps) = rule_noop.captures(def) {
                assert_eq!(caps.len(), 2);
                let id1 = parse_id(&caps[1]);
                matcher.add_regex(id, RegexCases::Noop(id1));
            } else if let Some(caps) = rule_union.captures(def) {
                assert_eq!(caps.len(), 3);
                let id1 = parse_id(&caps[1]);
                let id2 = parse_id(&caps[2]);
                matcher.add_regex(id, RegexCases::Union(id1, id2));
            } else if let Some(caps) = rule_concat.captures(def) {
                assert_eq!(caps.len(), 3);
                let id1 = parse_id(&caps[1]);
                let id2 = parse_id(&caps[2]);
                matcher.add_regex(id, RegexCases::Concat(id1, id2));
            } else if let Some(caps) = rule_union_concat.captures(def) {
                // In this case we generate two fresh IDs
                let fresh1 = fresh_id(id, 1);
                let fresh2 = fresh_id(id, 2);
                assert_eq!(caps.len(), 5);
                let id1 = parse_id(&caps[1]);
                let id2 = parse_id(&caps[2]);
                let id3 = parse_id(&caps[3]);
                let id4 = parse_id(&caps[4]);
                matcher.add_regex(id, RegexCases::Union(fresh1, fresh2));
                matcher.add_regex(fresh1, RegexCases::Concat(id1, id2));
                matcher.add_regex(fresh2, RegexCases::Concat(id3, id4));
            } else {
                panic!("Parsing error: could not parse rule: {}", def);
            }
        } else if line.is_empty() {
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

fn solve_part1(input_lines: &[String]) -> usize {
    let (mut matcher, msgs) = parse_input(input_lines);
    matcher.count_regex0_matches(&msgs)
}

fn solve_part2(input_lines: &[String]) -> usize {
    let (mut matcher, msgs) = parse_input(input_lines);

    // Additional rules:
    //     8: 42 | 42 8
    //     11: 42 31 | 42 11 31
    matcher.allow_loops();
    let fresh1 = fresh_id(8, 1);
    let fresh2 = fresh_id(11, 1);
    let fresh3 = fresh_id(11, 2);
    let fresh4 = fresh_id(11, 3);
    matcher.add_regex(8, RegexCases::Union(42, fresh1));
    matcher.add_regex(fresh1, RegexCases::Concat(42, 8));
    matcher.add_regex(11, RegexCases::Union(fresh2, fresh3));
    matcher.add_regex(fresh2, RegexCases::Concat(42, 31));
    matcher.add_regex(fresh3, RegexCases::Concat(42, fresh4));
    matcher.add_regex(fresh4, RegexCases::Concat(11, 31));

    matcher.count_regex0_matches(&msgs)
}

/*
    Tests and entrypoint
*/

#[cfg(test)]
mod tests {
    use super::*;
    use std::iter::once;

    struct Example {
        rules: &'static [&'static str],
        msgs: &'static [&'static str],
        loops: bool,
        expect: usize,
    }
    impl Example {
        fn check(&self) {
            println!(
                "===== Test with {} rules, {} msgs, loops: {}, expected: {}",
                self.rules.len(),
                self.msgs.len(),
                self.loops,
                self.expect
            );
            let lines: Vec<_> = self
                .rules
                .iter()
                .chain(once(&""))
                .chain(self.msgs.iter())
                .map(|s| s.to_string())
                .collect();
            let (mut matcher, msgs) = parse_input(&lines);
            if self.loops {
                matcher.allow_loops();
            }
            let ans = matcher.count_regex0_matches(&msgs);
            assert_eq!(ans, self.expect)
        }
    }

    const EX1: Example = Example {
        rules: &["0: 1 2", r#"1: "a""#, "2: 1 3 | 3 1", r#"3: "b""#],
        msgs: &["aab", "aba", "bba", "bab", "bbb", "aab"],
        loops: false,
        expect: 3,
    };
    const EX2: Example = Example {
        rules: &[
            "0: 4 10",
            "10: 1 5",
            "1: 2 3 | 3 2",
            "2: 4 4 | 5 5",
            "3: 4 5 | 5 4",
            r#"4: "a""#,
            r#"5: "b""#,
        ],
        msgs: &["ababbb", "bababa", "abbbab", "aaabbb", "aaaabbb"],
        loops: false,
        expect: 2,
    };
    const EX3: Example = Example {
        rules: &[
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            r#"1: "a""#,
            "11: 42 31",
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            r#"14: "b""#,
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "8: 42",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
        ],
        msgs: &[
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ],
        loops: false,
        expect: 3,
    };

    #[test]
    fn test_part1() {
        EX1.check();
        EX2.check();
        EX3.check();
    }

    const EX4: Example = Example {
        rules: &[
            "8: 42 | 108",
            "108: 42 8",
            "11: 42 31 | 42 111",
            "111: 11 31",
            "42: 9 14 | 10 1",
            "9: 14 27 | 1 26",
            "10: 23 14 | 28 1",
            r#"1: "a""#,
            "5: 1 14 | 15 1",
            "19: 14 1 | 14 14",
            "12: 24 14 | 19 1",
            "16: 15 1 | 14 14",
            "31: 14 17 | 1 13",
            "6: 14 14 | 1 14",
            "2: 1 24 | 14 4",
            "0: 8 11",
            "13: 14 3 | 1 12",
            "15: 1 | 14",
            "17: 14 2 | 1 7",
            "23: 25 1 | 22 14",
            "28: 16 1",
            "4: 1 1",
            "20: 14 14 | 1 15",
            "3: 5 14 | 16 1",
            "27: 1 6 | 14 18",
            r#"14: "b""#,
            "21: 14 1 | 1 14",
            "25: 1 1 | 1 14",
            "22: 14 14",
            "26: 14 22 | 1 20",
            "18: 15 15",
            "7: 14 5 | 1 21",
            "24: 14 1",
        ],
        msgs: &[
            "abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa",
            "bbabbbbaabaabba",
            "babbbbaabbbbbabbbbbbaabaaabaaa",
            "aaabbbbbbaaaabaababaabababbabaaabbababababaaa",
            "bbbbbbbaaaabbbbaaabbabaaa",
            "bbbababbbbaaaaaaaabbababaaababaabab",
            "ababaaaaaabaaab",
            "ababaaaaabbbaba",
            "baabbaaaabbaaaababbaababb",
            "abbbbabbbbaaaababbbbbbaaaababb",
            "aaaaabbaabaaaaababaa",
            "aaaabbaaaabbaaa",
            "aaaabbaabbaaaaaaabbbabbbaaabbaabaaa",
            "babaaabbbaaabaababbaabababaaab",
            "aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba",
        ],
        loops: true,
        expect: 12,
    };

    #[test]
    fn test_part2() {
        EX4.check();
    }
}

fn main() {
    let input_lines = file_to_vec("input/day19.txt");
    let part1 = solve_part1(&input_lines);
    let part2 = solve_part2(&input_lines);
    println!("Part 1 Answer: {}", part1);
    println!("Part 2 Answer: {}", part2);
}
