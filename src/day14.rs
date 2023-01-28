use itertools::Itertools;
use std::{collections::HashMap, ops::RangeInclusive};

#[derive(Eq, PartialEq, Clone, Debug)]
enum Cell {
    WALL,
    SAND,
}

type Part = i32;
type Coord = (i32, i32);
type Grid = HashMap<Coord, Cell>;

// Parses a string "number_a -> number_b" into a tuple of (i32, i32)
fn str_to_coord(s: &str) -> Coord {
    s.split(",")
        .map(|c| c.parse::<i32>().unwrap())
        .next_tuple()
        .unwrap()
}

fn parse_grid(buf: &[u8]) -> Grid {
    let mut grid = Grid::new();

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        for (a, b) in line.split(" -> ").tuple_windows() {
            let (x1, y1) = &str_to_coord(a);
            let (x2, y2) = &str_to_coord(b);

            for x in range(x1, x2) {
                for y in range(y1, y2) {
                    grid.insert((x, y), Cell::WALL);
                }
            }
        }
    }

    grid
}

fn range(x1: &i32, x2: &i32) -> RangeInclusive<i32> {
    if x1 <= x2 {
        return *x1..=*x2;
    } else {
        return *x2..=*x1;
    }
}

fn num_units(grid: &Grid) -> usize {
    grid.values().filter(|c| **c == Cell::SAND).count()
}

fn simulate(start: Coord, grid_orig: &Grid, max_y: i32, part: Part) -> usize {
    let mut grid = grid_orig.clone();
    let mut current = start.clone();
    loop {
        match current {
            (_, y) if y > max_y && part == 1 => {
                return num_units(&grid);
            }
            pos @ (_, y) if y == max_y + 1 && part == 2 => {
                grid.insert(pos, Cell::SAND);
                current = start;
            }
            pos @ (x, y) => {
                let down = (x, y + 1);
                match grid.get(&down) {
                    None => {
                        current = down;
                    }
                    Some(_) => {
                        let down_left = (x - 1, y + 1);
                        match grid.get(&down_left) {
                            None => {
                                current = down_left;
                            }
                            Some(_) => {
                                let down_right = (x + 1, y + 1);
                                match grid.get(&down_right) {
                                    None => {
                                        current = down_right;
                                    }
                                    Some(_) if pos == start && part == 2 => {
                                        grid.insert(pos, Cell::SAND);
                                        return num_units(&grid);
                                    }
                                    Some(_) => {
                                        grid.insert(pos, Cell::SAND);
                                        current = start;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

// fn print_grid(grid: &Grid) {
//     let min_x: i32 = *grid.keys().map(|(x, _y)| x).min().unwrap();
//     let max_x: i32 = *grid.keys().map(|(x, _y)| x).max().unwrap();
//     let min_y: i32 = *grid.keys().map(|(_x, y)| y).min().unwrap();
//     let max_y: i32 = *grid.keys().map(|(_x, y)| y).max().unwrap();

//     for y in min_y..=max_y {
//         for x in min_x..=max_x {
//             match grid.get(&(x, y)) {
//                 None => {
//                     print!(".");
//                 }
//                 Some(Cell::WALL) => {
//                     print!("#");
//                 }
//                 _ => panic!(),
//             }
//         }
//         println!()
//     }
// }

pub fn solve() -> (usize, usize) {
    let buf = include_bytes!("../inputs/input14.txt");
    let grid = parse_grid(buf);
    let max_y = *grid.keys().map(|(_x, y)| y).max().unwrap();
    let max_x = *grid.keys().map(|(x, _y)| x).max().unwrap();
    let start = (500, 0);
    println!("max = {},{}", max_x, max_y);
    let p1 = simulate(start, &grid, max_y, 1);
    let p2 = simulate(start, &grid, max_y, 2);
    (p1, p2)
}
