/*
    Advent of Code 2020
    Caleb Stanford
    Day 20 Solution
    2020-12-20

    Time (--release):
*/

use aoc2020::util::file_to_vec;
use std::collections::HashMap;

/*
    Tiles are stored as Boolean grids.

    Tiles support two functionalities:

    1. Tile edges can be summarized using EdgeInfo: this value is independent
       of the orientation of the tile and unique for a particular edge.
       In particular, the EdgeInfo is obtained by reading the edge as a binary
       integer in both forward directions, and taking the smaller of the two.
       EdgeInfo can then be used to identify corners and edge pieces, since they
       will have edges whose EdgeInfo is unique.
       This information is enough to solve part 1.

    2. Tiles can be rotated and reflected. This functionality is needed to
       assemble all of the tiles in the puzzle together to solve part 2.
*/

// Utility
fn bools_to_int<I: Iterator<Item = bool>>(v: I) -> usize {
    let bin_str = v.map(|b| if b { '1' } else { '0' }).collect::<String>();
    usize::from_str_radix(&bin_str, 2).unwrap()
}

#[derive(Clone, Copy, Debug)]
struct EdgeInfo {
    fwd_id: usize,
    bck_id: usize,
}
impl EdgeInfo {
    fn from_bools<I>(bools: I) -> Self
    where
        I: Clone + DoubleEndedIterator<Item = bool>,
    {
        let bools_rev = bools.clone().rev();
        let fwd_id = bools_to_int(bools);
        let bck_id = bools_to_int(bools_rev);
        Self { fwd_id, bck_id }
    }

    fn oriented_id(&self) -> usize {
        self.fwd_id
    }
    fn unoriented_id(&self) -> usize {
        self.fwd_id.min(self.bck_id)
    }
}

#[derive(Copy, Clone, Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    len: usize,
    grid: Vec<Vec<bool>>, // len x len grid
    times_reoriented: usize,
}
impl Tile {
    fn new(id: usize, grid: Vec<Vec<bool>>) -> Self {
        let len = grid.len();
        for row in &grid {
            assert_eq!(row.len(), len);
        }
        let times_reoriented = 0;
        Tile { id, len, grid, times_reoriented }
    }

    /* Edge getters */
    fn get_edge(&self, dir: Direction) -> EdgeInfo {
        // The final line of each case is the same, but it can't be
        // pulled out of the match due to type differences. (There may be
        // a way to do it with dynamic trait objects.)
        match dir {
            Direction::North => {
                let coords = (0..self.len).map(|j| (0, j));
                EdgeInfo::from_bools(coords.map(|(i, j)| self.grid[i][j]))
            }
            Direction::East => {
                let coords = (0..self.len).map(|i| (i, self.len - 1));
                EdgeInfo::from_bools(coords.map(|(i, j)| self.grid[i][j]))
            }
            Direction::South => {
                let coords = (0..self.len).map(|j| (self.len - 1, j));
                EdgeInfo::from_bools(coords.map(|(i, j)| self.grid[i][j]))
            }
            Direction::West => {
                let coords = (0..self.len).map(|i| (i, 0));
                EdgeInfo::from_bools(coords.map(|(i, j)| self.grid[i][j]))
            }
        }
    }

    fn get_edges(&self) -> [EdgeInfo; 4] {
        [
            self.get_edge(Direction::North),
            self.get_edge(Direction::East),
            self.get_edge(Direction::South),
            self.get_edge(Direction::West),
        ]
    }

    /* Rotation and reflection */
    fn rotate(&mut self) {
        let mut new_self = self.clone();
        for i in 0..self.len {
            for j in 0..self.len {
                new_self.grid[j][self.len - i - 1] = self.grid[i][j];
            }
        }
        *self = new_self;
    }
    fn reflect(&mut self) {
        let mut new_self = self.clone();
        for i in 0..self.len {
            for j in 0..self.len {
                new_self.grid[j][i] = self.grid[i][j];
            }
        }
        *self = new_self;
    }
    fn reorient(&mut self) {
        self.rotate();
        self.times_reoriented += 1;
        if self.times_reoriented % 4 == 0 {
            self.reflect();
        }
    }

    /* Check fitting together with another tile */
    fn fits_core(
        &self,
        other: &Self,
        dir1: Direction,
        dir2: Direction,
    ) -> bool {
        self.get_edge(dir1).oriented_id() == other.get_edge(dir2).oriented_id()
    }
    fn fits_south(&self, other: &Self) -> bool {
        self.fits_core(other, Direction::South, Direction::North)
    }
    fn fits_east(&self, other: &Self) -> bool {
        self.fits_core(other, Direction::East, Direction::West)
    }
}

/*
    A Puzzle is a collection of Tiles.

    The puzzle keeps track of EdgeInfo for each tile so that they can be matched up.
    For part 1, the puzzle first determines which tiles are corners, edges, and
    inside pieces.
*/
#[derive(Debug, Default)]
struct Puzzle {
    tiles: HashMap<usize, Tile>,       // tile ID -> tile
    edges: HashMap<usize, Vec<usize>>, // *unoriented* edge ID -> tile IDs
    corner_tiles: Vec<usize>,
    edge_tiles: Vec<usize>,
    inside_tiles: Vec<usize>,
    tile_len: usize,
    puzzle_len: usize,
    assembled_tiles: Vec<Vec<usize>>,
    assembled_picture: Vec<Vec<bool>>,
}
impl Puzzle {
    fn is_puzzle_edge(&self, edge: &EdgeInfo) -> bool {
        let edge_count = self.edges[&edge.unoriented_id()].len();
        // Important check: each unoriented ID uniquely implies either
        // a single tile or a pair of tiles
        assert!(edge_count == 1 || edge_count == 2);
        edge_count == 1
    }
    fn get_other_at_edge(&self, tile_id: usize, edge: &EdgeInfo) -> usize {
        assert!(!self.is_puzzle_edge(edge));
        let possibilities = &self.edges[&edge.unoriented_id()];
        if possibilities[0] == tile_id {
            possibilities[1]
        } else {
            possibilities[0]
        }
    }

    fn sort_tiles(&mut self, tile_list: &[Tile]) {
        // Store tiles and edges
        for tile in tile_list {
            if self.tile_len == 0 {
                self.tile_len = tile.len;
            } else {
                assert_eq!(self.tile_len, tile.len);
            }
            self.tiles.insert(tile.id, tile.clone());
            for &info in &tile.get_edges() {
                let edge = self.edges.entry(info.unoriented_id()).or_default();
                edge.push(tile.id);
            }
        }
        // Identify tile types and separate
        for tile in tile_list {
            let mut unique_edges = 0;
            for info in &tile.get_edges() {
                if self.is_puzzle_edge(info) {
                    unique_edges += 1;
                }
            }
            match unique_edges {
                0 => self.inside_tiles.push(tile.id),
                1 => self.edge_tiles.push(tile.id),
                2 => self.corner_tiles.push(tile.id),
                _ => panic!("Found tile with three unique edges: {:?}", tile),
            }
        }
        // Calculate puzzle dimensions (assume a square)
        self.puzzle_len = (self.edge_tiles.len() / 4) + 2;
        self.check_tile_counts();
    }

    fn check_tile_counts(&self) {
        let n = self.puzzle_len;
        assert_eq!(self.corner_tiles.len(), 4);
        assert_eq!(self.edge_tiles.len(), 4 * (n - 2));
        assert_eq!(self.inside_tiles.len(), (n - 2) * (n - 2));
        assert_eq!(self.edges.len(), 2 * n * (n + 1));
    }

    fn print_tile_counts(&self) {
        println!("=== Puzzle counts ===");
        println!("Corner tiles: {}", self.corner_tiles.len());
        println!("Edge tiles: {}", self.edge_tiles.len());
        println!("Inside tiles: {}", self.inside_tiles.len());
        println!("Total tiles: {}", self.tiles.len());
        println!("Unique tile edge patterns: {}", self.edges.len());
    }

    fn assemble_top_left(&mut self, tl_id: usize) {
        while !self
            .is_puzzle_edge(&self.tiles[&tl_id].get_edge(Direction::South))
            || !self
                .is_puzzle_edge(&self.tiles[&tl_id].get_edge(Direction::East))
        {
            self.tiles.get_mut(&tl_id).unwrap().reorient();
        }
    }
    fn assemble_south(&mut self, id1: usize, id2: usize) {
        while !self.tiles[&id1].fits_south(&self.tiles[&id2]) {
            self.tiles.get_mut(&id2).unwrap().reorient();
        }
    }
    fn assemble_east(&mut self, id1: usize, id2: usize) {
        while !self.tiles[&id1].fits_east(&self.tiles[&id2]) {
            self.tiles.get_mut(&id2).unwrap().reorient();
        }
    }
    fn assemble_puzzle(&mut self) {
        self.assembled_tiles = vec![vec![0; self.puzzle_len]; self.puzzle_len];
        for i in 0..self.puzzle_len {
            for j in 0..self.puzzle_len {
                // Assemble tile (i, j)
                if i == 0 && j == 0 {
                    let top_left = self.corner_tiles[0];
                    self.assemble_top_left(top_left);
                    self.assembled_tiles[0][0] = top_left;
                } else if j == 0 {
                    let placed_id = self.assembled_tiles[i - 1][j];
                    let south_edge =
                        self.tiles[&placed_id].get_edge(Direction::South);
                    let next_id =
                        self.get_other_at_edge(placed_id, &south_edge);
                    self.assemble_south(placed_id, next_id);
                } else {
                    let placed_id = self.assembled_tiles[i][j - 1];
                    let east_edge =
                        self.tiles[&placed_id].get_edge(Direction::East);
                    let next_id = self.get_other_at_edge(placed_id, &east_edge);
                    self.assemble_east(placed_id, next_id);
                }
            }
        }
    }

    fn assemble_picture(&mut self) {
        unimplemented!();
    }

    fn new(tile_list: &[Tile]) -> Self {
        let mut puzzle: Self = Default::default();
        puzzle.sort_tiles(tile_list);
        // TODO: debug & implement for part 2
        // puzzle.assemble_puzzle();
        // puzzle.assemble_picture();
        puzzle
    }

    fn part1_answer(&self) -> usize {
        self.corner_tiles.iter().product()
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
        #[allow(clippy::needless_range_loop)]
        for j in (i + 1)..(i + 11) {
            let bools: Vec<bool> = lines[j]
                .chars()
                .map(|ch| match ch {
                    '#' => true,
                    '.' => false,
                    _ => panic!("invalid tile char: {}", ch),
                })
                .collect();
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
