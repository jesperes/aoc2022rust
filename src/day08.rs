use std::cmp::max;

const WIDTH: i32 = 99;
const HEIGHT: i32 = 99;

type Grid = Vec<u8>;

fn read_grid(grid: &Grid, x: i32, y: i32) -> char {
    grid[(y * (WIDTH + 1) + x) as usize] as char
}

fn is_valid_coord(x: i32, y: i32) -> bool {
    x >= 0 && x < WIDTH && y >= 0 && y < HEIGHT
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input08.txt");
    let grid = buf.to_vec();
    let mut p1: i64 = 0;
    let mut p2: i64 = 0;

    for x in 0..WIDTH {
        for y in 0..HEIGHT {
            if is_tree_visible(x, y, &grid) {
                p1 += 1;
            }

            p2 = max(p2, scenic_score(x, y, &grid))
        }
    }

    (p1, p2)
}

fn is_tree_visible(x: i32, y: i32, grid: &Grid) -> bool {
    let deltas: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let th = read_grid(grid, x, y);

    for (dx, dy) in deltas {
        let mut x0 = x + dx;
        let mut y0 = y + dy;

        loop {
            if !is_valid_coord(x0, y0) {
                return true;
            } else {
                if read_grid(grid, x0, y0) >= th {
                    break;
                } else {
                    x0 += dx;
                    y0 += dy;
                }
            }
        }
    }

    false
}

fn scenic_score(x: i32, y: i32, grid: &Grid) -> i64 {
    let deltas: Vec<(i32, i32)> = vec![(1, 0), (0, 1), (-1, 0), (0, -1)];
    let mut score: i64 = 1;
    let th = read_grid(grid, x, y);

    for (dx, dy) in deltas {
        let mut x0 = x + dx;
        let mut y0 = y + dy;
        let mut dist = 0;

        loop {
            if !is_valid_coord(x0, y0) {
                score *= dist;
                break;
            } else {
                if read_grid(grid, x0, y0) >= th {
                    dist += 1;
                    score *= dist;
                    break;
                } else {
                    x0 += dx;
                    y0 += dy;
                    dist += 1;
                }
            }
        }
    }

    score
}

#[cfg(test)]
mod tests {
    #[test]
    fn read_grid() {
        let buf = include_bytes!("../inputs/input08.txt");
        let grid = buf.to_vec();
        assert_eq!('2', super::read_grid(&grid, 0, 0));
        assert_eq!('3', super::read_grid(&grid, 0, 1));
        assert_eq!('1', super::read_grid(&grid, 0, 98));
        assert_eq!('3', super::read_grid(&grid, 98, 98));
    }
}
