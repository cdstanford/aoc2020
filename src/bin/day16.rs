/*
    Advent of Code 2020
    Caleb Stanford
    Day 16 Solution
    2020-12-16

    Start time: 11:41am
    Solved part 1: 1:41pm (2 hr)
    Solved part 2: 4:25pm (2 hr, 44 min)
    Code cleanup: 4:55pm

    Time (--release): 0m0.082s
*/

use aoc2020::util::{file_to_vec, iter_to_pair};
use z3::ast::Bool;
use z3::{Config, Context, SatResult, Solver};

/*
    Struct to capture range constraints (e.g. 1-5 or 10-20 or 50-60)

    Inspecting the input, all numbers are small (between 1 and 999), and the
    range constraint boundaries are statically known.
    Therefore the best way to store range constraints (unions of ranges) should
    just be a vector<bool> of length 1000, not something fancier like a sorted
    list of the range boundaries.
*/
const GLOBAL_UB: usize = 1000;
struct Ranges {
    set: [bool; GLOBAL_UB],
}
impl Ranges {
    // Constructors
    fn new_empty() -> Self {
        Self { set: [false; GLOBAL_UB] }
    }
    fn from_range(low: usize, high: usize) -> Self {
        // Inclusive
        let mut result = Self::new_empty();
        for i in low..=high {
            result.set[i] = true;
        }
        result
    }
    // Membership check
    fn contains(&self, i: usize) -> bool {
        debug_assert!(i < GLOBAL_UB);
        self.set[i]
    }
    // Combining ranges (immutably)
    fn union(&self, other: &Self) -> Self {
        let mut result = Self::new_empty();
        for i in 0..GLOBAL_UB {
            result.set[i] = self.contains(i) || other.contains(i)
        }
        result
    }
}

/*
    Bipartite matching finder (for part 2)

    Input: a square Boolean matrix of which inputs can match with which outputs
    Output: A list of the output indices corresponding to each input index.

    We outsource the constraint solving to Z3.
*/
fn find_matching(matchable: &[Vec<bool>]) -> Vec<usize> {
    let n = matchable.len();
    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    // One variable per row, column (possible match)
    let vars: Vec<Vec<_>> = (0..20)
        .map(|i| {
            (0..20)
                .map(|j| Bool::new_const(&ctx, format!("match_{}_{}", i, j)))
                .collect()
        })
        .collect();
    let var_refs: Vec<Vec<_>> =
        vars.iter().map(|vars_i| vars_i.iter().collect()).collect();
    assert_eq!(vars.len(), n);
    assert_eq!(var_refs.len(), n);

    // Variables conform to matchable constraints
    for i in 0..n {
        for j in 0..n {
            if !matchable[i][j] {
                solver.assert(&var_refs[i][j].not());
            }
        }
    }

    // At least one match per row
    for var_row in &var_refs {
        solver.assert(&Bool::or(&ctx, var_row));
    }

    // At most one match per column
    for j in 0..n {
        for i1 in 0..n {
            for i2 in (i1 + 1)..n {
                let both_i1_i2 =
                    Bool::and(&ctx, &[var_refs[i1][j], var_refs[i2][j]]);
                solver.assert(&both_i1_i2.not());
            }
        }
    }

    // Solve
    // println!("Solver: {}", solver);
    match solver.check() {
        SatResult::Sat => {
            let model = solver.get_model().unwrap();
            // println!("Model: {:?}", model);
            vars.iter()
                .map(|var_row| {
                    let matches: Vec<_> = var_row
                        .iter()
                        .enumerate()
                        .filter(|&(_i, var)| {
                            model.eval(var).unwrap().as_bool().unwrap()
                        })
                        .map(|(i, _var)| i)
                        .collect();
                    assert_eq!(matches.len(), 1);
                    matches[0]
                })
                .collect()
        }
        SatResult::Unsat => {
            let unsat_core = solver.get_unsat_core();
            println!("Unsat core: {:?}", unsat_core);
            panic!("Constraints were unsatisfiable");
        }
        SatResult::Unknown => {
            panic!("Z3 failed to solve constraints");
        }
    }
}

/*
    Part 1
*/
fn merge_constraints(fields: &[(String, Ranges)]) -> Ranges {
    fields
        .iter()
        .map(|(_field_name, r)| r)
        .fold(Ranges::new_empty(), |r1, r2| r1.union(r2))
}
fn invalid_fields(ticket: &[usize], constraints: &Ranges) -> Vec<usize> {
    ticket.iter().filter(|&&n| !constraints.contains(n)).cloned().collect()
}
fn solve_part1(fields: &[(String, Ranges)], tickets: &[Vec<usize>]) -> usize {
    let constraints = merge_constraints(fields);
    tickets.iter().flat_map(|ticket| invalid_fields(ticket, &constraints)).sum()
}

/*
    Part 2
*/
fn field_matches(
    valid_tickets: &[Vec<usize>],
    index: usize,
    constraints: &Ranges,
) -> bool {
    for ticket in valid_tickets {
        if !constraints.contains(ticket[index]) {
            return false;
        }
    }
    true
}
fn solve_part2(
    fields: &[(String, Ranges)],
    tickets: &[Vec<usize>],
    your_ticket: &[usize],
) -> usize {
    let constraints = merge_constraints(fields);
    let valid_tickets: Vec<Vec<usize>> = tickets
        .iter()
        .filter(|ticket| invalid_fields(ticket, &constraints).is_empty())
        .cloned()
        .collect();
    let mut field_possibilities = vec![vec![]; 20];
    for field in 0..20 {
        for index in 0..20 {
            field_possibilities[field].push(field_matches(
                &valid_tickets,
                index,
                &fields[field].1,
            ));
        }
    }
    // Find bipartite matching
    // println!("Matchable: {:?}", field_possibilities);
    let matching = find_matching(&field_possibilities);
    println!("Part 2 Matching: {:?}", matching);
    // Find the six fields starting with "departure" and compute answer
    let departure_fields: Vec<usize> = (0..20)
        .filter(|&f| fields[f].0.split(' ').next().unwrap() == "departure")
        .map(|f| matching[f])
        .collect();
    assert_eq!(departure_fields.len(), 6);
    departure_fields.iter().map(|&f| your_ticket[f]).product()
}

/*
    Parsing and entrypoint
*/
fn parse_field(line: &str) -> (String, Ranges) {
    let (field_name, split0) = iter_to_pair(line.split(": "));
    let (split1, split2) = iter_to_pair(split0.split(" or "));
    let (low1, high1) =
        iter_to_pair(split1.split('-').map(|n| n.parse().unwrap()));
    let (low2, high2) =
        iter_to_pair(split2.split('-').map(|n| n.parse().unwrap()));

    let range1 = Ranges::from_range(low1, high1);
    let range2 = Ranges::from_range(low2, high2);
    let ranges = range1.union(&range2);

    (field_name.to_owned(), ranges)
}
fn parse_ticket(line: &str) -> Vec<usize> {
    let result: Vec<usize> =
        line.split(',').map(|n| n.parse().unwrap()).collect();
    assert_eq!(result.len(), 20);
    result
}
fn main() {
    let lines = file_to_vec("input/day16.txt");

    let fields: Vec<(String, Ranges)> =
        lines[0..20].iter().map(|s| s as &str).map(parse_field).collect();
    assert_eq!(fields.len(), 20);

    assert_eq!(lines[20], "");
    assert_eq!(lines[21], "your ticket:");
    let your_ticket: Vec<usize> = parse_ticket(&lines[22]);

    assert_eq!(lines[23], "");
    assert_eq!(lines[24], "nearby tickets:");
    let tickets: Vec<Vec<usize>> =
        lines[25..].iter().map(|s| s as &str).map(parse_ticket).collect();

    println!("Part 1 Answer: {}", solve_part1(&fields, &tickets));
    println!("Part 2 Answer: {}", solve_part2(&fields, &tickets, &your_ticket));
}
