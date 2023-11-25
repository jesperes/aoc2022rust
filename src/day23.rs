use std::collections::HashMap;

use fnv::FnvHashSet;

type Pos = (i32, i32);

const NORTH: i32 = 0;
const SOUTH: i32 = 1;
const WEST: i32 = 2;
const EAST: i32 = 3;

fn do_one_round(elves: &mut FnvHashSet<Pos>, round: i32) -> bool {
    let mut elfmap: HashMap<Pos, Pos> = HashMap::new();
    let mut movemap: HashMap<Pos, i32> = HashMap::new();

    for elf in elves.iter() {
        let possible_moves = possible_moves(&elf, round, &elves);
        let num_moves = possible_moves.len();
        if num_moves == 0 || num_moves == 4 {
            continue;
        } else {
            let proposed_move = possible_moves.first().unwrap();
            elfmap.insert(*elf, *proposed_move);
            *movemap.entry(*proposed_move).or_insert(0) += 1;
        }
    }

    let mut non_conflicting_moves: Vec<(Pos, Pos)> = Vec::new();

    for (elf, move_to) in elfmap {
        if (*movemap.get(&move_to).unwrap_or(&0)) < 2 {
            non_conflicting_moves.push((elf, move_to));
        }
    }

    if non_conflicting_moves.len() == 0 {
        false
    } else {
        for (from, to) in non_conflicting_moves {
            elves.remove(&from);
            elves.insert(to);
        }
        true
    }
}

fn possible_moves(elf: &Pos, round: i32, elves: &FnvHashSet<Pos>) -> Vec<Pos> {
    let (x, y) = elf;
    let mut moves: Vec<Pos> = Vec::new();
    for n in 0..4 {
        let dir = (round + n) % 4;
        let adj = match dir {
            NORTH => {
                let py = y - 1;
                vec![(x - 1, py), (*x, py), (x + 1, py)]
            }
            SOUTH => {
                let py = y + 1;
                vec![(x - 1, py), (*x, py), (x + 1, py)]
            }
            WEST => {
                let px = x - 1;
                vec![(px, y - 1), (px, *y), (px, y + 1)]
            }
            EAST => {
                let px = x + 1;
                vec![(px, y - 1), (px, *y), (px, y + 1)]
            }
            _ => unreachable!(),
        };

        if !adj.iter().any(|elem| elves.contains(elem)) {
            moves.push(match dir {
                NORTH => (*x, y - 1),
                SOUTH => (*x, y + 1),
                WEST => (x - 1, *y),
                EAST => (x + 1, *y),
                _ => unreachable!(),
            });
        }
    }
    return moves;
}

fn find_p1(elves: &FnvHashSet<Pos>) -> i32 {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for (x, y) in elves {
        min_x = (*x).min(min_x);
        max_x = (*x).max(max_x);
        min_y = (*y).min(min_y);
        max_y = (*y).max(max_y);
    }

    (max_x - min_x + 1) * (max_y - min_y + 1) - elves.len() as i32
}

pub fn solve() -> (i32, i32) {
    let bytes = include_bytes!("../inputs/input23.txt");
    let mut elves: FnvHashSet<Pos> = FnvHashSet::default();

    String::from_utf8_lossy(bytes)
        .split("\n")
        .enumerate()
        .for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, c)| {
                if c == '#' {
                    elves.insert((x as i32, y as i32));
                }
            });
        });

    let mut p1 = 0;
    let mut p2 = 0;

    for round in 0.. {
        if round == 10 {
            p1 = find_p1(&elves);
        }

        if !do_one_round(&mut elves, round) {
            p2 = round + 1;
            break;
        }
    }

    (p1, p2)
}
