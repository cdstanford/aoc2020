/*
    Advent of Code 2020
    Caleb Stanford
    Day 20 Solution
    2020-12-20 -- 2020-12-22

    Time (--release): 0m0.055s
*/

use aoc2020::util::file_to_vec;
use std::collections::HashMap;

/*
    Tiles are stored as Boolean grids. They support the following:

    - Naming edges:
      EdgeInfo can be used to give unique names to tile edges, considering them
      to be either oriented or unoriented. The unique unoriented value is just
      the min of the oriented value and its reverse.
      Collecting all of the unoriented values is enough for part 1, as this
      allows detecting which tiles are corners.

    - Rotation and reflection:
      This is needed in part 2 to assemble all the tiles in the puzzle together.
      We can iterate over all 8 rotations and reflections by repeatedly
      calling .reorient().

    - Assembling:
      Check if the tile fits together with another tile along a given direction.
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
use Direction::{East, North, South, West};

#[derive(Clone, Debug)]
struct Tile {
    id: usize,
    len: usize,
    grid: Vec<Vec<bool>>, // len x len grid
    times_reoriented: usize,
}
const TILE_DISPLAY_MAX_ROWS: usize = 7;
const TILE_DISPLAY_MAX_COLS: usize = 50;
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
    fn edge_from_coords<I>(&self, coords: I) -> EdgeInfo
    where
        I: Clone + DoubleEndedIterator<Item = (usize, usize)>,
    {
        EdgeInfo::from_bools(coords.map(|(i, j)| self.grid[i][j]))
    }
    fn get_edge(&self, dir: Direction) -> EdgeInfo {
        let n = self.len;
        match dir {
            North => self.edge_from_coords((0..n).map(|j| (0, j))),
            East => self.edge_from_coords((0..n).map(|i| (i, n - 1))),
            South => self.edge_from_coords((0..n).map(|j| (n - 1, j))),
            West => self.edge_from_coords((0..n).map(|i| (i, 0))),
        }
    }
    fn get_edges(&self) -> [EdgeInfo; 4] {
        [
            self.get_edge(North),
            self.get_edge(East),
            self.get_edge(South),
            self.get_edge(West),
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
        self.fits_core(other, South, North)
    }
    fn fits_east(&self, other: &Self) -> bool {
        self.fits_core(other, East, West)
    }

    // Printing
    fn print(&self) {
        for row in self.grid.iter().take(TILE_DISPLAY_MAX_ROWS) {
            for pixel in row.iter().take(TILE_DISPLAY_MAX_COLS) {
                match pixel {
                    true => print!("#"),
                    false => print!("."),
                };
            }
            if self.len > TILE_DISPLAY_MAX_COLS {
                print!(" …");
            }
            println!();
        }
        if self.len > TILE_DISPLAY_MAX_ROWS {
            println!("         …  …  …");
        }
    }
}

/*
    Puzzle types for sorting and assembling the puzzle

    I previously had a single monolithic Puzzle struct for all stages of
    assembly, but this seems like bad design: the data model is completely
    different before tiles are sorted in, after tiles are assembled into
    a puzzle, and after the puzzle is assembled into an image.
    Therefore, we instead adopt a more functional style with types:
        - UnsortedPuzzle: a collection of tiles
        - SortedPuzzle: tiles sorted into useful categories
          (Part 1 is solved at this stage)
        - AssembledPuzzle: tiles assembled into the correct grid
        - AssembledImage: the image extracted from the assembled puzzle
          (Part 2 is solved at this stage)
*/

#[derive(Debug, Default)]
struct UnsortedPuzzle {
    tiles: HashMap<usize, Tile>,       // tile ID -> tile
    edges: HashMap<usize, Vec<usize>>, // *unoriented* edge ID -> tile IDs
    tile_len: usize,
}
#[derive(Debug, Default)]
struct SortedPuzzle {
    corner_tiles: Vec<Tile>,
    edge_tiles: Vec<Tile>,
    inside_tiles: Vec<Tile>,
    puzzle_len: usize,
}
#[derive(Debug)]
struct AssembledPuzzle {
    grid: Vec<Vec<Tile>>,
    tile_len: usize,
    puzzle_len: usize,
}
#[derive(Clone, Debug)]
struct AssembledImage(Tile);

/*
    Collecting the puzzle tiles to store the set of corresponding edges
*/
impl UnsortedPuzzle {
    fn new(tile_list: &[Tile]) -> Self {
        let mut puzzle: Self = Default::default();
        for tile in tile_list {
            // All puzzle tiles should be the same length
            if puzzle.tile_len == 0 {
                puzzle.tile_len = tile.len;
            } else {
                assert_eq!(puzzle.tile_len, tile.len);
            }
            // Store tile and its edges
            puzzle.tiles.insert(tile.id, tile.clone());
            for &info in &tile.get_edges() {
                let entry = puzzle.edges.entry(info.unoriented_id());
                entry.or_default().push(tile.id);
            }
        }
        puzzle
    }

    fn is_puzzle_edge(&self, edge: &EdgeInfo) -> bool {
        let edge_count = self.edges[&edge.unoriented_id()].len();
        // Important check: each unoriented ID uniquely implies either
        // a single tile or a pair of tiles
        assert!(edge_count == 1 || edge_count == 2);
        edge_count == 1
    }
    fn get_other_at_edge(&self, tile: &Tile, edge: &EdgeInfo) -> Tile {
        assert!(!self.is_puzzle_edge(edge));
        let possibilities = &self.edges[&edge.unoriented_id()];
        if possibilities[0] == tile.id {
            self.tiles[&possibilities[1]].clone()
        } else {
            self.tiles[&possibilities[0]].clone()
        }
    }

    fn print_tile_counts(&self) {
        println!("Total tiles: {}", self.tiles.len());
        println!("Unique tile edge patterns: {}", self.edges.len());
    }
}

/*
    Sorting the tiles
*/
impl SortedPuzzle {
    fn new(unsorted: &UnsortedPuzzle) -> Self {
        let mut puzzle: Self = Default::default();
        for tile in unsorted.tiles.values() {
            let mut unique_edges = 0;
            for info in &tile.get_edges() {
                if unsorted.is_puzzle_edge(info) {
                    unique_edges += 1;
                }
            }
            match unique_edges {
                2 => puzzle.corner_tiles.push(tile.clone()),
                1 => puzzle.edge_tiles.push(tile.clone()),
                0 => puzzle.inside_tiles.push(tile.clone()),
                _ => panic!("Found tile with three unique edges: {:?}", tile),
            }
        }
        // Calculate puzzle dimensions (assume a square)
        puzzle.puzzle_len = (puzzle.edge_tiles.len() / 4) + 2;
        puzzle.check_tile_counts(unsorted);
        puzzle
    }

    fn check_tile_counts(&self, unsorted: &UnsortedPuzzle) {
        let n = self.puzzle_len;
        assert_eq!(self.corner_tiles.len(), 4);
        assert_eq!(self.edge_tiles.len(), 4 * (n - 2));
        assert_eq!(self.inside_tiles.len(), (n - 2) * (n - 2));
        assert_eq!(unsorted.tiles.len(), n * n);
        assert_eq!(unsorted.edges.len(), 2 * n * (n + 1));
    }
    fn print_tile_counts(&self) {
        println!("Corner tiles: {}", self.corner_tiles.len());
        println!("Edge tiles: {}", self.edge_tiles.len());
        println!("Inside tiles: {}", self.inside_tiles.len());
    }
}

/*
    Assembling the tiles (in top-to-bottom, left-to-right order)
*/
fn fits_southeast(
    unsorted: &UnsortedPuzzle,
    above: Option<&Tile>,
    left: Option<&Tile>,
    tile: &Tile,
) -> bool {
    (above.is_some() || unsorted.is_puzzle_edge(&tile.get_edge(North)))
        && (above.is_none() || above.unwrap().fits_south(tile))
        && (left.is_some() || unsorted.is_puzzle_edge(&tile.get_edge(West)))
        && (left.is_none() || left.unwrap().fits_east(tile))
}
fn assemble_tile(
    unsorted: &UnsortedPuzzle,
    above: Option<&Tile>,
    left: Option<&Tile>,
    tile: &mut Tile,
) {
    // Precondition: there exists a unique orientation that fits
    while !fits_southeast(unsorted, above, left, tile) {
        tile.reorient();
    }
}
impl AssembledPuzzle {
    fn new(unsorted: &UnsortedPuzzle, sorted: &SortedPuzzle) -> Self {
        let tile_len = unsorted.tile_len;
        let puzzle_len = sorted.puzzle_len;
        let mut grid: Vec<Vec<Tile>> = Vec::new(); // n x n grid
        for i in 0..puzzle_len {
            grid.push(Vec::new());
            for j in 0..puzzle_len {
                // Assemble tile (i, j)
                if i == 0 && j == 0 {
                    let mut new_tile = sorted.corner_tiles[0].clone();
                    assemble_tile(unsorted, None, None, &mut new_tile);
                    grid[i].push(new_tile);
                } else if j == 0 {
                    // i > 0, j == 0
                    let above = &grid[i - 1][j];
                    let edge = above.get_edge(South);
                    let mut new_tile = unsorted.get_other_at_edge(above, &edge);
                    assemble_tile(unsorted, Some(above), None, &mut new_tile);
                    grid[i].push(new_tile);
                } else {
                    // j > 0
                    let left = &grid[i][j - 1];
                    let edge = left.get_edge(East);
                    let mut new_tile = unsorted.get_other_at_edge(left, &edge);
                    let above =
                        if i == 0 { None } else { Some(&grid[i - 1][j]) };
                    assemble_tile(unsorted, above, Some(left), &mut new_tile);
                    grid[i].push(new_tile);
                }
                debug_assert_eq!(grid[i][j].len, tile_len);
            }
            debug_assert_eq!(grid[i].len(), puzzle_len);
        }
        debug_assert_eq!(grid.len(), puzzle_len);
        Self { grid, tile_len, puzzle_len }
    }
    fn print_ids(&self) {
        for row in &self.grid {
            for tile in row {
                print!("{} ", tile.id);
            }
            println!();
        }
    }
}

/*
    Assembling the image
*/
impl AssembledImage {
    fn new(assembled: &AssembledPuzzle) -> Self {
        let canvas_step = assembled.tile_len - 2;
        let tile_last = assembled.tile_len - 1;
        let puzzle_len = assembled.puzzle_len;
        let canvas_size = canvas_step * puzzle_len;
        let mut canvas: Vec<Vec<bool>> =
            vec![vec![false; canvas_size]; canvas_size];
        for row in 0..puzzle_len {
            for col in 0..puzzle_len {
                let tile = &assembled.grid[row][col];
                // Sanity check: verify that the pixels line up
                if row > 0 {
                    let prev = &assembled.grid[row - 1][col];
                    for j in 0..canvas_step {
                        assert_eq!(tile.grid[0][j], prev.grid[tile_last][j]);
                    }
                }
                if col > 0 {
                    let prev = &assembled.grid[row][col - 1];
                    for i in 0..canvas_step {
                        assert_eq!(tile.grid[i][0], prev.grid[i][tile_last]);
                    }
                }
                // Copy over other pixels
                for i in 0..canvas_step {
                    for j in 0..canvas_step {
                        let x = canvas_step * row + i;
                        let y = canvas_step * col + j;
                        let pixel = tile.grid[i + 1][j + 1];
                        canvas[x][y] = pixel;
                    }
                }
            }
        }
        Self(Tile::new(0, canvas))
    }
}

/*
    Detecting sea monsters

    Sea monster image:
    ----------------------
    |                  # |
    |#    ##    ##    ###|
    | #  #  #  #  #  #   |
    ----------------------
    3 x 20
*/

const SEAMONSTER_COORDS: &[(usize, usize)] = &[
    (1, 0),
    (2, 1),
    (2, 4),
    (1, 5),
    (1, 6),
    (2, 7),
    (2, 10),
    (1, 11),
    (1, 12),
    (2, 13),
    (2, 16),
    (1, 17),
    (1, 18),
    (0, 18),
    (1, 19),
];

impl AssembledImage {
    fn seamonster_at(&self, i: usize, j: usize) -> bool {
        if i + 2 >= self.0.len || j + 19 >= self.0.len {
            return false;
        }
        for (di, dj) in SEAMONSTER_COORDS {
            if !self.0.grid[i + di][j + dj] {
                return false;
            }
        }
        true
    }
    fn count_seamonsters(&self) -> usize {
        let mut count = 0;
        for i in 0..self.0.len {
            for j in 0..self.0.len {
                if self.seamonster_at(i, j) {
                    count += 1;
                }
            }
        }
        count
    }
    fn find_seamonster_orientation(&mut self) {
        while self.count_seamonsters() == 0 {
            self.0.reorient();
        }
        let seamonsters = self.count_seamonsters();
        self.0.reorient();
        for _ in 0..7 {
            assert_eq!(self.count_seamonsters(), 0);
            self.0.reorient();
        }
        assert_eq!(seamonsters, self.count_seamonsters());
    }

    fn erase_seamonster_at(&mut self, i: usize, j: usize) {
        for (di, dj) in SEAMONSTER_COORDS {
            self.0.grid[i + di][j + dj] = false;
        }
    }
    fn erase_all_seamonsters(&self) -> Self {
        let mut other = self.clone();
        for i in 0..self.0.len {
            for j in 0..self.0.len {
                if self.seamonster_at(i, j) {
                    other.erase_seamonster_at(i, j);
                }
            }
        }
        other
    }

    fn print(&self) {
        self.0.print();
        println!("Seamonsters found: {}", self.count_seamonsters());
    }
}

/*
    Answers, input parsing, and entrypoint
*/

fn part1_answer(sorted: &SortedPuzzle) -> usize {
    sorted.corner_tiles.iter().map(|c| c.id).product()
}

fn part2_answer(clean: &AssembledImage) -> usize {
    clean.0.grid.iter().flatten().filter(|&&p| p).count()
}

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
        for j in 1..=10 {
            let bools: Vec<bool> = lines[i + j]
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

    println!("=== Unsorted puzzle ===");
    let unsorted = UnsortedPuzzle::new(&tile_list);
    unsorted.print_tile_counts();

    println!("=== Sorted puzzle ===");
    let sorted = SortedPuzzle::new(&unsorted);
    sorted.print_tile_counts();

    println!("=== Solved puzzle ===");
    let assembled = AssembledPuzzle::new(&unsorted, &sorted);
    assembled.print_ids();

    println!("=== Assembled image (oriented) ===");
    let mut image = AssembledImage::new(&assembled);
    image.find_seamonster_orientation();
    image.print();

    println!("=== Seamonster-free image ===");
    let clean = image.erase_all_seamonsters();
    clean.print();

    println!("=== Answers ===");
    println!("Part 1: {}", part1_answer(&sorted));
    println!("Part 2: {}", part2_answer(&clean));
}
