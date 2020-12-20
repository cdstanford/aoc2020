/*
    Advent of Code 2020
    Caleb Stanford
    Day 20 Solution
    2020-12-20

    Start: 1:17pm

    Time (--release):
*/

use aoc2020::util::file_to_vec;
use std::collections::HashMap;

/*
    Tiles are stored as Boolean grids.

    Tile edges can be summarized using EdgeInfo: this value is independent
    of the orientation of the tile and unique for a particular edge.
    In particular, the EdgeInfo is obtained by reading the edge as a binary
    integer in both forward directions, and taking the smaller of the two.

    EdgeInfo can then be used to identify corners and edge pieces, since they
    will have edges whose EdgeInfo is unique.
*/
type EdgeInfo = usize;
#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    len: usize,
    grid: Vec<Vec<bool>>, // len x len grid
}
impl Tile {
    fn new(id: usize, grid: Vec<Vec<bool>>) -> Self {
        let len = grid.len();
        for row in &grid {
            assert_eq!(row.len(), len);
        }
        Tile { id, len, grid }
    }

    /* Calculating unique edge identifiers */
    fn bools_to_int<I: Iterator<Item = bool>>(v: I) -> usize {
        let bin_str = v.map(|b| if b { '1' } else { '0' }).collect::<String>();
        usize::from_str_radix(&bin_str, 2).unwrap()
    }
    fn get_edge_info<I>(&self, coords: I) -> EdgeInfo
    where
        I: Clone + DoubleEndedIterator<Item = (usize, usize)>
    {
        let bools_fwd = coords.clone().map(|(i, j)| self.grid[i][j]);
        let bools_bck = coords.clone().rev().map(|(i, j)| self.grid[i][j]);
        debug_assert_eq!(coords.clone().count(), self.len);
        let int_fwd = Self::bools_to_int(bools_fwd);
        let int_bck = Self::bools_to_int(bools_bck);
        int_fwd.min(int_bck)
    }
    fn get_edges_info(&self) -> [EdgeInfo; 4] {
        let edge1_coords = (0..self.len).map(|i| (i, 0));
        let edge2_coords = (0..self.len).map(|j| (0, j));
        let edge3_coords = (0..self.len).map(|i| (i, self.len - 1));
        let edge4_coords = (0..self.len).map(|j| (self.len - 1, j));
        [
            self.get_edge_info(edge1_coords),
            self.get_edge_info(edge2_coords),
            self.get_edge_info(edge3_coords),
            self.get_edge_info(edge4_coords),
        ]
    }
}

/*
    A Puzzle is a collection of Tiles.

    The puzzle keeps track of EdgeInfo for each tile so that they can be matched up.
    For part 1, the puzzle first determines which tiles are corners, edges, and
    inside pieces.
*/
#[derive(Debug)]
struct Puzzle {
    edge_counts: HashMap<EdgeInfo, usize>,
    tiles: HashMap<usize, Tile>,
    corner_tiles: Vec<Tile>,
    edge_tiles: Vec<Tile>,
    inside_tiles: Vec<Tile>,
}
impl Puzzle {
    fn new(tile_list: &[Tile]) -> Self {
        // Store tiles in a map by ID
        // Store a count of edge patterns
        let mut tiles = HashMap::new();
        let mut edge_counts = HashMap::new();
        for tile in tile_list {
            tiles.insert(tile.id, tile.clone());
            for &info in &tile.get_edges_info() {
                let edge_count = edge_counts.entry(info).or_insert(0);
                *edge_count += 1;
            }
        }
        // Identify tile types
        let mut inside_tiles = Vec::new();
        let mut edge_tiles = Vec::new();
        let mut corner_tiles = Vec::new();
        for tile in tile_list {
            let mut unique = 0;
            for info in &tile.get_edges_info() {
                let edge_count = edge_counts[info];
                assert!(edge_count >= 1 && edge_count <= 2);
                if edge_count == 1 {
                    unique += 1;
                }
            }
            match unique {
                0 => inside_tiles.push(tile.clone()),
                1 => edge_tiles.push(tile.clone()),
                2 => corner_tiles.push(tile.clone()),
                _ => panic!("Found tile with three unique edges: {:?}", tile),
            }
        }
        assert_eq!(corner_tiles.len(), 4);
        Self { tiles, edge_counts, corner_tiles, edge_tiles, inside_tiles }
    }

    fn print_tile_counts(&self) {
        println!("=== Puzzle tile counts ===");
        println!("Corners: {}", self.corner_tiles.len());
        println!("Edges: {}", self.edge_tiles.len());
        println!("Inside pieces: {}", self.inside_tiles.len());
        println!("Total: {}", self.tiles.len());
    }

    fn part1_answer(&self) -> usize {
        self.corner_tiles.iter().map(|c| c.id).product()
    }

    fn part2_answer(&self) -> usize {
        0
    }

}

/*
    Input parsing and entrypoint
*/

fn parse_input(lines: &[String]) -> Vec<Tile> {
    let mut result = Vec::new();
    let mut i = 0;
    assert_eq!(lines.len() % 12, 0);
    while i < lines.len() {
        assert_eq!(&lines[i][0..5], "Tile ");
        assert_eq!(&lines[i][9..10], ":");
        assert_eq!(&lines[i + 11], "");
        let tile_id = lines[i][5..9].parse::<usize>().unwrap();
        let mut grid = Vec::new();
        for j in (i + 1)..(i + 11) {
            let bools: Vec<bool> = lines[j].chars().map(|ch| match ch {
                '#' => true,
                '.' => false,
                _ => panic!("invalid tile char: {}", ch),
            }).collect();
            assert_eq!(bools.len(), 10);
            grid.push(bools);
        }
        assert_eq!(grid.len(), 10);
        result.push(Tile::new(tile_id, grid));
        i += 12;
    }
    result
}

fn main() {
    let tile_list = parse_input(&file_to_vec("input/day20.txt"));
    // println!("Tiles: {:?}", tile_list);
    // println!("First tile: {:?}", tile_list[0]);

    let puzzle = Puzzle::new(&tile_list);
    puzzle.print_tile_counts();

    println!("=== Answers ===");
    println!("Part 1: {}", puzzle.part1_answer());
    println!("Part 2: {}", puzzle.part2_answer());
}
