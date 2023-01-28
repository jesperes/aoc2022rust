use grid::*;
use itertools::Itertools;
use std::cmp::*;
use std::ops::RangeInclusive;

#[derive(Eq, PartialEq, Clone, Debug, Default)]
enum Cell {
    WALL,
    SAND,
    #[default]
    EMPTY,
}

type Part = i32;
type Coord = (usize, usize);
type CellGrid = Grid<Cell>;

// Parses a string "number_a -> number_b" into a tuple of (i32, i32)
fn str_to_coord(s: &str) -> Coord {
    s.split(",")
        .map(|c| c.parse::<usize>().unwrap())
        .next_tuple()
        .unwrap()
}

fn parse_grid(buf: &[u8]) -> (CellGrid, usize) {
    let rows = 200;
    let cols = 700;
    let mut grid = CellGrid::new(rows, cols);
    let mut max_y = 0;

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        for (a, b) in line.split(" -> ").tuple_windows() {
            let (x1, y1) = &str_to_coord(a);
            let (x2, y2) = &str_to_coord(b);

            for x in range(x1, x2) {
                for y in range(y1, y2) {
                    grid[y][x] = Cell::WALL;
                    max_y = max(y, max_y)
                }
            }
        }
    }

    (grid, max_y)
}

fn range(x1: &usize, x2: &usize) -> RangeInclusive<usize> {
    if x1 <= x2 {
        return *x1..=*x2;
    } else {
        return *x2..=*x1;
    }
}

fn simulate(start: Coord, grid_orig: &CellGrid, max_y: usize, part: Part) -> usize {
    let mut grid = grid_orig.clone();
    let mut current = start.clone();
    let mut num_units = 0;
    loop {
        match current {
            (_, y) if y > max_y && part == 1 => {
                return num_units;
            }
            (x, y) if y == max_y + 1 && part == 2 => {
                grid[y][x] = Cell::SAND;
                num_units += 1;
                current = start;
            }
            pos @ (x, y) => match grid[y + 1][x] {
                Cell::EMPTY => {
                    current = (x, y + 1);
                }
                _ => match grid[y + 1][x - 1] {
                    Cell::EMPTY => {
                        current = (x - 1, y + 1);
                    }
                    _ => match grid[y + 1][x + 1] {
                        Cell::EMPTY => {
                            current = (x + 1, y + 1);
                        }
                        _ if pos == start && part == 2 => {
                            grid[y][x] = Cell::SAND;
                            num_units += 1;
                            return num_units;
                        }
                        _ => {
                            grid[y][x] = Cell::SAND;
                            num_units += 1;
                            current = start;
                        }
                    },
                },
            },
        }
    }
}

pub fn solve() -> (usize, usize) {
    let buf = include_bytes!("../inputs/input14.txt");
    let (grid, max_y) = parse_grid(buf);
    let start = (500, 0);
    let p1 = simulate(start, &grid, max_y, 1);
    let p2 = simulate(start, &grid, max_y, 2);
    (p1, p2)
}
