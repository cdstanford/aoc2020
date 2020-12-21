/*
    Advent of Code 2020
    Caleb Stanford
    Day 21 Solution
    2020-12-21

    Start: 12:15pm
    Solved part 1: 2:17pm
    Solved part 2: 3:37pm

    Time (--release): 0m0.051s
*/

use aoc2020::util::{file_to_vec, iter_to_pair};
use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;
use std::iter::{once, FromIterator};

/*
    InjectionFinder: a data structure for finding injections from U into V,
    given constraints on which Vs can be assigned to each u in U.

    This is similar to Day 16 part 2. We could re-use the code for that to
    find the bipartite matching (the find_matching function), but we instead
    write a direct algorithm.
*/

#[derive(Debug, Default)]
struct InjectionFinder<U, V> {
    fwd: HashMap<U, HashSet<V>>,
    bck: HashMap<V, HashSet<U>>,
}
impl<U, V> InjectionFinder<U, V>
where
    U: Clone + Debug + Eq + Hash + PartialEq,
    V: Clone + Debug + Eq + Hash + PartialEq,
{
    // Getters
    fn u_is_seen(&self, u: &U) -> bool {
        self.fwd.contains_key(u)
    }
    fn v_is_seen(&self, v: &V) -> bool {
        self.bck.contains_key(v)
    }
    fn u_iter(&self) -> impl Iterator<Item = &U> {
        self.fwd.keys()
    }
    fn v_iter(&self) -> impl Iterator<Item = &V> {
        self.bck.keys()
    }
    fn u_degree(&self, u: &U) -> usize {
        debug_assert!(self.u_is_seen(u));
        self.fwd.get(u).unwrap().len()
    }
    fn v_degree(&self, v: &V) -> usize {
        debug_assert!(self.v_is_seen(v));
        self.bck.get(v).unwrap().len()
    }
    // Get the match, if unique
    fn u_match(&self, u: &U) -> Option<V> {
        if self.u_degree(u) == 1 {
            Some(self.fwd.get(u).unwrap().iter().next().unwrap().clone())
        } else {
            None
        }
    }
    fn v_match(&self, v: &V) -> Option<U> {
        if self.v_degree(v) == 1 {
            Some(self.bck.get(v).unwrap().iter().next().unwrap().clone())
        } else {
            None
        }
    }

    // Internal modification to the graph -- not to be called directly
    // Could also write ensure_u(&mut self, u: &U), but currently unneeded
    fn ensure_v(&mut self, v: &V) {
        if !self.v_is_seen(v) {
            self.bck.insert(v.clone(), HashSet::new());
        }
        debug_assert!(self.v_is_seen(v));
    }
    fn add_edge_core(&mut self, u: &U, v: &V) {
        self.fwd.entry(u.clone()).or_default().insert(v.clone());
        self.bck.entry(v.clone()).or_default().insert(u.clone());
    }
    fn remove_edge_core(&mut self, u: &U, v: &V) {
        debug_assert!(self.u_is_seen(u));
        debug_assert!(self.v_is_seen(v));
        self.fwd.get_mut(u).unwrap().remove(v);
        self.bck.get_mut(v).unwrap().remove(u);
    }

    // Exposed constraint API
    // Add constraints on an input vertex u
    // A constraint is of the form (u, V_u) and states that u maps to one of
    // the elements V_u.
    fn add_constraint(&mut self, u: &U, v_set: &HashSet<V>) {
        // Ensure everything exiests
        for v in v_set {
            self.ensure_v(v);
        }
        // Add constraint on u
        if self.u_is_seen(u) {
            // Narrow existing possibilities
            let mut to_remove = Vec::new();
            for v in self.fwd.get(u).unwrap().iter() {
                if !v_set.contains(v) {
                    to_remove.push(v.clone());
                }
            }
            for v in &to_remove {
                self.remove_edge_core(u, v);
            }
        } else {
            // Add starting possibilities
            for v in v_set {
                self.add_edge_core(u, v);
            }
        }
    }

    // Part 1
    // Check if there is an injection containing v
    // For this, it is sufficient to see if there is an edge (u, v) for any u.
    // The proof of this uses the assumption that at least one injection exists
    // (ensured by the problem input). If there is no edge containing v, then
    // of course there can be no injection. If there is an edge containing v,
    // then consider the injection that exists. If it doesn't contain v, we can
    // modify it by re-assigning u to v, and it is still a valid injection.
    fn exists_injection_containing(&self, v: &V) -> bool {
        debug_assert!(self.v_is_seen(v));
        !self.bck.get(v).unwrap().is_empty()
    }

    // Part 2
    // Solve the constraints to find an injection.
    // For this, it is sufficient to identify a u that corresponds to only one
    // v, assign it, and repeat. If there is no such u, one can show that the
    // matching is not unique. Rough argument: construct a cycle alternating
    // between edges in the matching and not. Reassigning the edges in the
    // cycle gives a new matching.
    // Vice versa works also, i.e. identifying a forced v instead of a u.
    // Note: we could use a priority queue / heap for a more efficient
    // implementation here (O(1) to find the u with only 1 corresponding v.)
    // This solution is worst-case O(n^2).
    fn solve(&mut self) {
        let mut unmatched: HashSet<V> =
            self.v_iter().filter(|&v| self.v_degree(v) >= 1).cloned().collect();
        while !unmatched.is_empty() {
            let mut matched = Vec::new();
            for v in &unmatched {
                debug_assert!(self.v_degree(v) >= 1);
                if self.v_degree(v) == 1 {
                    // remove all other edges
                    let u = self.v_match(v).unwrap();
                    self.add_constraint(&u, &once(v.clone()).collect());
                    matched.push(v.clone());
                }
            }
            for v in &matched {
                unmatched.remove(v);
            }
        }
        self.check_solution();
    }
    fn check_solution(&self) {
        for u in self.u_iter() {
            debug_assert_eq!(self.u_degree(u), 1);
        }
        for v in self.v_iter() {
            debug_assert!(self.v_degree(v) <= 1);
        }
    }
}

/*
    Parsing and entrypoint

    We wrap String in types Ingredient and Allergen for the benefit of static
    typing (so that we don't accidentally mix up the ordering).
    Allergen needs to deriving Ord for part 2.
*/

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
struct Ingredient(String);
#[derive(Clone, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Allergen(String);

type Constraint = (Vec<Ingredient>, Vec<Allergen>);
type Solver = InjectionFinder<Allergen, Ingredient>;

fn parse_input_line(line: &str) -> Constraint {
    let (s1, s23) = iter_to_pair(line.split(" (contains "));
    let (s2, s3) = iter_to_pair(s23.split(')'));
    assert_eq!(s3, "");
    let ingredients =
        s1.split(' ').map(|s| Ingredient(s.to_string())).collect();
    let allergens = s2.split(", ").map(|s| Allergen(s.to_string())).collect();
    (ingredients, allergens)
}
fn parse_input(lines: &[String]) -> Vec<Constraint> {
    lines.iter().map(|s| s.as_ref()).map(parse_input_line).collect()
}
fn create_inj_finder(constraints: &[Constraint]) -> Solver {
    let mut inj_finder: Solver = Default::default();
    for (ingredients, allergens) in constraints {
        let ingredient_set = HashSet::from_iter(ingredients.iter().cloned());
        for allergen in allergens {
            inj_finder.add_constraint(allergen, &ingredient_set);
        }
    }
    inj_finder
}

fn solve_part1(constraints: &[Constraint], inj_finder: &Solver) -> usize {
    // Total ingredients which can't contain an allergen
    let mut count = 0;
    for (ingredients, _) in constraints {
        for ingredient in ingredients {
            if !inj_finder.exists_injection_containing(ingredient) {
                count += 1;
            }
        }
    }
    count
}

fn solve_part2(inj_finder: &mut Solver) -> String {
    inj_finder.solve();
    let mut pairs: Vec<(Allergen, Ingredient)> = inj_finder
        .u_iter()
        .map(|u| (u.clone(), inj_finder.u_match(u).unwrap()))
        .collect();
    pairs.sort_by_key(|(u, _v)| u.clone());
    let dangerous_ingreds: Vec<String> =
        pairs.iter().map(|(_u, v)| v.0.clone()).collect();
    dangerous_ingreds.join(",")
}

fn main() {
    let lines = file_to_vec("input/day21.txt");
    let constraints = parse_input(&lines);
    let mut inj_finder = create_inj_finder(&constraints);

    println!("Part 1 Answer: {}", solve_part1(&constraints, &inj_finder));
    println!("Part 2 Answer: {}", solve_part2(&mut inj_finder));
}
