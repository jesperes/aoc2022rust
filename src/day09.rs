// use itertools::Itertools;
use std::collections::HashSet;

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input09.txt");
    let mut instrs: Vec<(char, i32)> = vec![];

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let mut words = line.split(' ');
        let dir: char = words.next().unwrap().chars().next().unwrap();
        let n = words.next().unwrap().parse::<i32>().unwrap();
        instrs.push((dir, n));
    }

    let p1: i64 = simulate(&instrs, 2);
    let p2: i64 = simulate(&instrs, 10);
    (p1, p2)
}

fn simulate(instrs: &Vec<(char, i32)>, num_knots: i32) -> i64 {
    let mut rope: Vec<(i32, i32)> = vec![];
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    for _ in 0..num_knots {
        rope.push((0, 0));
    }

    for (dir, n) in instrs {
        for _ in 0..*n {
            move_head(&mut rope[0], *dir);

            for i in 1..num_knots {
                let i0 = i as usize;
                rope[i0] = move_follow(&rope[i0 - 1], &rope[i0]);
            }

            visited.insert(rope.last().unwrap().clone());
        }
    }

    visited.len() as i64
}

fn move_head(head: &mut (i32, i32), dir: char) {
    match dir {
        'U' => head.1 -= 1,
        'D' => head.1 += 1,
        'L' => head.0 -= 1,
        'R' => head.0 += 1,
        _ => panic!(),
    }
}

fn move_follow(h: &(i32, i32), t: &(i32, i32)) -> (i32, i32) {
    if is_touching(h, t) {
        return *t;
    }

    match (h, t) {
        (h, t) if t.0 == h.0 && h.1 < t.1 => (t.0, t.1 - 1),
        (h, t) if t.0 < h.0 && h.1 < t.1 => (t.0 + 1, t.1 - 1),
        (h, t) if t.0 < h.0 && h.1 == t.1 => (t.0 + 1, t.1),
        (h, t) if t.0 < h.0 && h.1 > t.1 => (t.0 + 1, t.1 + 1),
        (h, t) if t.0 == h.0 && h.1 > t.1 => (t.0, t.1 + 1),
        (h, t) if t.0 > h.0 && h.1 > t.1 => (t.0 - 1, t.1 + 1),
        (h, t) if t.0 > h.0 && h.1 == t.1 => (t.0 - 1, t.1),
        (h, t) if t.0 > h.0 && h.1 < t.1 => (t.0 - 1, t.1 - 1),
        _ => panic!(),
    }
}

fn is_touching(c1: &(i32, i32), c2: &(i32, i32)) -> bool {
    let dist_x = (c1.0 - c2.0).abs();
    let dist_y = (c1.1 - c2.1).abs();
    if dist_x >= 2 || dist_y >= 2 {
        false
    } else {
        true
    }
}
