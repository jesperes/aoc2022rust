use grid::Grid;
use lazy_regex::regex;

const GRID_COLS: i32 = 150;
const GRID_ROWS: i32 = 200;

type Direction = u8;
const RIGHT: u8 = 0;
const DOWN: u8 = 1;
const LEFT: u8 = 2;
const UP: u8 = 3;

#[derive(Debug)]
enum Instr {
    Left,
    Right,
    Walk(i32),
}

#[derive(Debug)]
struct Pos {
    row: usize,
    col: usize,
}

impl Pos {
    fn new(row: usize, col: usize) -> Self {
        Pos { row, col }
    }
}

struct State {
    pos: Pos,
    heading: Direction,
}

impl State {
    fn new(start_pos: &Pos) -> State {
        State {
            pos: Pos::new(start_pos.row, start_pos.col),
            heading: RIGHT,
        }
    }
    fn execute(&mut self, instr: &Instr, grid: &Grid<char>, part: i32) {
        match instr {
            Instr::Left => self.heading = (self.heading + 3) % 4,
            Instr::Right => self.heading = (self.heading + 1) % 4,
            Instr::Walk(steps) => self.walk(steps, grid, part),
        }
    }

    fn walk(&mut self, steps: &i32, grid: &Grid<char>, part: i32) {
        for _ in 0..*steps {
            if let Some((new_pos, tile)) = get_forward_pos(&self.pos, self.heading, &grid) {
                match tile {
                    '.' => {
                        self.pos = new_pos;
                        continue;
                    }
                    '#' => break,
                    _ => unreachable!(),
                }
            } else {
                // Moving one step would move us outside the known grid, so
                // we need to warp.
                let (warp_pos, warp_heading) = self.warp_pos(part, grid);
                match grid[warp_pos.row][warp_pos.col] {
                    '.' => {
                        self.pos = warp_pos;
                        self.heading = warp_heading;
                        continue;
                    }
                    '#' => break,
                    _ => unreachable!(),
                }
            };
        }
    }

    fn warp_pos(&mut self, part: i32, grid: &Grid<char>) -> (Pos, Direction) {
        if part == 1 {
            let mut warp_pos = Pos::new(self.pos.row, self.pos.col);
            let warp_heading = (self.heading + 2) % 4; // turn backwards
            loop {
                if let Some((pos, _tile)) = get_forward_pos(&warp_pos, warp_heading, grid) {
                    warp_pos = pos;
                    continue;
                } else {
                    return (warp_pos, self.heading);
                }
            }
        } else if part == 2 {
            // Part 2. Handcoded rules for warping around the sides of the cube.
            // There are 14 of these, corresponding to the 14 "open" edges in
            // the flattened cube.
            //
            // The mix of (x, y) and (row, col) notation here is because this
            // part is a direct translation of my Erlang solution where I used
            // (x, y)-notation.
            let (x, y, warp_heading) = match (self.pos.col, self.pos.row, self.heading) {
                (x, y, LEFT) if x == 0 && y <= 149 => (50, 149 - y, RIGHT),
                (x, y, RIGHT) if x == 149 => (99, 149 - y, LEFT),
                (x, y, LEFT) if x == 50 && y <= 49 => (0, 149 - y, RIGHT),
                (x, y, UP) if y == 0 && x <= 99 => (0, x + 100, RIGHT),
                (x, y, UP) if y == 0 && x >= 100 => (x - 100, 199, UP),
                (x, y, UP) if y == 100 && x <= 49 => (50, x + 50, RIGHT),
                (x, y, LEFT) if x == 50 && y >= 50 && y <= 99 => (y - 50, 100, DOWN),
                (x, y, LEFT) if x == 0 && y >= 150 => (y - 100, 0, DOWN),
                (x, y, DOWN) if y == 199 => (x + 100, 0, DOWN),
                (x, y, RIGHT) if x == 49 && y >= 150 => (y - 100, 149, UP),
                (x, y, DOWN) if y == 149 && x >= 50 => (49, x + 100, LEFT),
                (x, y, RIGHT) if x == 99 && y >= 50 && y <= 99 => (y + 50, 49, UP),
                (x, y, DOWN) if x >= 100 && y == 49 => (99, x - 50, LEFT),
                (x, y, RIGHT) if x == 99 && y >= 100 => (149, 149 - y, LEFT),
                _ => unreachable!(),
            };
            (Pos::new(y, x), warp_heading)
        } else {
            unreachable!()
        }
    }
}

// Return the position "forward" from current pos, if the position
// is on the grid. If the position requires warping, returns None.
fn get_forward_pos(pos: &Pos, heading: Direction, grid: &Grid<char>) -> Option<(Pos, char)> {
    let (row, col) = match heading {
        RIGHT => (pos.row as i32, pos.col as i32 + 1),
        DOWN => (pos.row as i32 + 1, pos.col as i32),
        LEFT => (pos.row as i32, pos.col as i32 - 1),
        UP => (pos.row as i32 - 1, pos.col as i32),
        _ => unreachable!(),
    };

    // Since the (row,col) arguments are usize, we need to take care
    // not to try to convert negative numbers to rows and cols.
    match (row, col) {
        (r, c) if r < 0 || c < 0 || c >= GRID_COLS || r >= GRID_ROWS => None,
        (r, c) => match grid[r as usize][c as usize] {
            tile if tile == '.' || tile == '#' => Some((Pos::new(r as usize, c as usize), tile)),
            _ => None,
        },
    }
}

pub fn solve() -> (i64, i64) {
    let bytes = include_bytes!("../inputs/input22.txt");
    let str = String::from_utf8_lossy(bytes);
    let lines: Vec<String> = str.split("\n").map(|s| s.to_string()).collect();
    let mut grid: Grid<char> = Grid::new(GRID_ROWS as usize, GRID_COLS as usize);
    let start_pos = Pos::new(0, str.find('.').unwrap());

    for (row, line) in lines.iter().enumerate() {
        if row >= GRID_ROWS as usize {
            // remaining rows are part of the instruction block
            break;
        }
        for (col, c) in line.chars().enumerate() {
            grid[row][col] = c
        }
    }

    let instrs: Vec<Instr> = regex!(r"(\d+|[RL])")
        .find_iter(&str)
        .map(|m| match m.as_str() {
            "L" => Instr::Left,
            "R" => Instr::Right,
            walk => Instr::Walk(walk.parse().unwrap()),
        })
        .collect();

    let p1 = walk(&start_pos, &instrs, &grid, 1);
    let p2 = walk(&start_pos, &instrs, &grid, 2);
    (p1, p2)
}

fn walk(start_pos: &Pos, instrs: &Vec<Instr>, grid: &Grid<char>, part: i32) -> i64 {
    let mut state = State::new(start_pos);
    for instr in instrs {
        state.execute(instr, grid, part)
    }
    to_password(&state)
}

fn to_password(state: &State) -> i64 {
    ((state.pos.row + 1) * 1000 + (state.pos.col + 1) * 4 + state.heading as usize) as i64
}
