/*
    Advent of Code 2020
    Caleb Stanford
    Day 7 Solution
    2020-12-07
*/

use aoc2020::util::{file_to_vec, line_to_words};
use std::collections::{HashMap, HashSet};
use std::hash::Hash;

/*
    BagGraph<V> gives a basic implementation of a directed multi-graph:
    a bag contains a multiset of other bags.

    It stores edges in both directions, and uses Clone on V to simplify
    ownership issues.

    It implements a basic DFS reachability search for part 1,
    and querying the size of a bag (number of bags inside) for part 2.

    Note: we assume that the graph is acylic for both part 1 and part 2.
*/

#[derive(Clone, Debug)]
struct BagGraph<V> {
    bags: HashSet<V>,
    bag_sources: HashMap<V, Vec<V>>,
    bag_targets: HashMap<V, Vec<V>>,
    bags_inside_memo: HashMap<V, usize>, // memoization for part 2
}
impl<V> BagGraph<V>
where
    V: Clone + Eq + Hash + PartialEq,
{
    fn new() -> Self {
        Self {
            bags: HashSet::new(),
            bag_sources: HashMap::new(),
            bag_targets: HashMap::new(),
            bags_inside_memo: HashMap::new(),
        }
    }
    fn add_bag(&mut self, v: &V) {
        if self.bags.insert(v.clone()) {
            self.bag_sources.insert(v.clone(), Vec::new());
            self.bag_targets.insert(v.clone(), Vec::new());
        }
    }
    fn add_edge(&mut self, v1: &V, v2: &V) {
        self.add_bag(v1);
        self.add_bag(v2);
        self.bag_sources.get_mut(&v2).unwrap().push(v1.clone());
        self.bag_targets.get_mut(&v1).unwrap().push(v2.clone());
    }

    // For part 1: reachability analysis using DFS
    fn dfs(edges: &HashMap<V, Vec<V>>, start: &V) -> Vec<V> {
        let mut visited = HashSet::new();
        let mut to_visit = Vec::new();
        let mut result = Vec::new();
        to_visit.push(start);
        while !to_visit.is_empty() {
            let u = to_visit.pop().unwrap();
            if !visited.contains(u) {
                result.push(u.clone());
                visited.insert(u);
                for v in edges.get(u).unwrap() {
                    to_visit.push(v);
                }
            }
        }
        result
    }
    fn reachable_to(&self, sink: &V) -> HashSet<V> {
        Self::dfs(&self.bag_sources, sink).into_iter().collect()
    }
    fn count_reachable_inclusive(&self, sink: &V) -> usize {
        self.reachable_to(sink).len()
    }
    fn count_reachable(&self, sink: &V) -> usize {
        // subtract one for this bag itself
        // Note: this assumes acyclicity
        self.count_reachable_inclusive(sink) - 1
    }

    // For part 2: querying number of bags
    fn bags_inside(&mut self, bag: &V) -> usize {
        if self.bags_inside_memo.contains_key(bag) {
            *self.bags_inside_memo.get(bag).unwrap()
        } else {
            let answer = self.bags_inside_rec(bag);
            self.bags_inside_memo.insert(bag.clone(), answer);
            answer
        }
    }
    fn bags_inside_inclusive(&mut self, bag: &V) -> usize {
        // including this bag itself
        self.bags_inside(bag) + 1
    }
    fn bags_inside_rec(&mut self, bag: &V) -> usize {
        // Note: this is recursive, assumes acyclicity and will loop forever
        // otherwise
        let mut total = 0;
        let nested_bags = self.bag_targets.get(bag).unwrap().clone();
        for nested_bag in nested_bags {
            total += self.bags_inside_inclusive(&nested_bag);
        }
        total
    }
}

fn solve_part1(bag_graph: &BagGraph<String>) -> usize {
    let shiny_gold = "shiny gold".to_owned();
    bag_graph.count_reachable(&shiny_gold)
}

fn solve_part2(bag_graph: &mut BagGraph<String>) -> usize {
    let shiny_gold = "shiny gold".to_owned();
    bag_graph.bags_inside(&shiny_gold)
}

fn main() {
    // Parse input
    let mut bag_graph = BagGraph::new();
    for line in file_to_vec("input/day7.txt") {
        let words = line_to_words(&line);
        let name = format!("{} {}", words[0], words[1]);
        assert_eq!("bags", words[2]);
        assert_eq!("contain", words[3]);
        // Insert vertex
        bag_graph.add_bag(&name);
        // Two cases: "X Y bags contain no other bags" vs contains a list
        if words.len() != 7 {
            assert!(words.len() % 4 == 0);
            for i in 1..(words.len() / 4) {
                let item_num = words[4 * i].parse::<usize>().unwrap();
                let item_name =
                    format!("{} {}", words[4 * i + 1], words[4 * i + 2]);
                let rem = words[4 * i + 3].as_str();
                assert!(vec!["bag,", "bags,", "bag.", "bags."].contains(&rem));
                // Insert edges
                for _i in 0..item_num {
                    bag_graph.add_edge(&name, &item_name);
                }
            }
        }
    }
    // println!("Bag Graph: {:?}", bag_graph);
    // Solve
    println!("Part 1 Answer: {}", solve_part1(&bag_graph));
    println!("Part 2 Answer: {}", solve_part2(&mut bag_graph));
}
