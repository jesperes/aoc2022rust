use std::collections::{BTreeSet, HashMap};

const WIDTH: usize = 64;
const HEIGHT: usize = 40;
const LOWEST_ELEV: i32 = 'a' as i32;
const HIGHEST_ELEV: i32 = 'z' as i32;

type Coord = (i64, i64);

struct Grid {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: Coord,
    end: Coord,
}

impl Grid {
    fn new_from_bytes(buf: &[u8]) -> Grid {
        let mut grid: Vec<Vec<char>> = Vec::new();
        let mut start: Coord = (0, 0);
        let mut end: Coord = (0, 0);

        for y in 0..HEIGHT {
            grid.push(Vec::new());

            assert_eq!('\n', buf[WIDTH] as char);

            for x in 0..WIDTH {
                let c: char = buf[(y * (WIDTH + 1) + x) as usize] as char;
                grid[y as usize].push(c);
                if c == 'S' {
                    start = (x as i64, y as i64);
                } else if c == 'E' {
                    end = (x as i64, y as i64);
                }
            }
        }

        Grid {
            grid,
            width: WIDTH,
            height: HEIGHT,
            start,
            end,
        }
    }

    fn is_valid_coord(&self, pos: Coord) -> bool {
        let x = pos.0;
        let y = pos.1;
        x >= 0 && x < (self.width as i64) && y >= 0 && y < (self.height as i64)
    }

    fn elevation_at(&self, pos: Coord) -> i32 {
        assert!(self.is_valid_coord(pos));
        if pos == self.start {
            LOWEST_ELEV
        } else if pos == self.end {
            HIGHEST_ELEV
        } else {
            self.grid[pos.1 as usize][pos.0 as usize] as i32
        }
    }

    // A* search from a set of start coordinates
    fn find(&self, starts: Vec<Coord>) -> i64 {
        let mut open_set: BTreeSet<(i64, Coord)> = BTreeSet::new();
        let mut g_score: HashMap<Coord, i64> = HashMap::new();

        for s in starts {
            open_set.insert((self.dist(s, self.end), s));
            g_score.insert(s, 0);
        }

        loop {
            let (_, current) = open_set.pop_first().unwrap();
            if current == self.end {
                return *g_score.get(&current).unwrap();
            } else {
                for nbr in self.neighbors(current) {
                    let new_g_score: i64 = g_score.get(&current).unwrap() + 1;
                    let nbr_g_score: i64 = *g_score.get(&nbr).unwrap_or(&(1000 as i64));
                    if new_g_score < nbr_g_score {
                        open_set.insert((new_g_score + self.dist(nbr, self.end), nbr));
                        g_score.insert(nbr, new_g_score);
                    }
                }
            }
        }
    }

    fn dist(&self, a: Coord, b: Coord) -> i64 {
        return (a.0 - b.0).abs() + (a.1 - b.1).abs();
    }

    fn neighbors(&self, node: Coord) -> Vec<Coord> {
        let mut nbrs: Vec<Coord> = Vec::new();
        let elevation = self.elevation_at(node);
        let max_allowed_elevation = elevation + 1;

        for c in [(0, 1), (1, 0), (-1, 0), (0, -1)] {
            let nbr = (node.0 + c.0, node.1 + c.1);
            if self.is_valid_coord(nbr) && self.elevation_at(nbr) <= max_allowed_elevation {
                nbrs.push(nbr);
            }
        }
        nbrs
    }
}

fn find_all_as(grid: &Grid) -> Vec<Coord> {
    let mut a_vec: Vec<Coord> = Vec::new();
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let pos = (x as i64, y as i64);
            if grid.elevation_at(pos) == LOWEST_ELEV {
                a_vec.push(pos);
            }
        }
    }
    a_vec
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input12.txt");
    let grid: Grid = Grid::new_from_bytes(buf);

    let p1 = grid.find(vec![grid.start]);
    let p2 = grid.find(find_all_as(&grid));

    (p1, p2)
}
