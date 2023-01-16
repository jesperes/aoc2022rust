use std::collections::HashSet;

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct Coord {
    x: i32,
    y: i32,
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input09.txt");
    let p1: i64 = simulate(buf, 2);
    let p2: i64 = simulate(buf, 10);
    (p1, p2)
}

fn simulate(buf: &[u8], num_knots: i32) -> i64 {
    let mut rope: Vec<Coord> = vec![];
    let mut visited: HashSet<Coord> = HashSet::new();

    for _ in 0..num_knots {
        rope.push(Coord { x: 0, y: 0 });
    }

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let mut words = line.split(' ');
        let dir: char = words.next().unwrap().chars().next().unwrap();
        let n = words.next().unwrap().parse::<i32>().unwrap();

        for _ in 0..n {
            move_head(&mut rope[0], dir);

            for i in 1..num_knots {
                move_follow(&mut rope, i as usize);
            }
            visited.insert(rope.last().unwrap().clone());
        }
    }

    visited.len() as i64
}

fn move_head(head: &mut Coord, dir: char) {
    match dir {
        'U' => head.y -= 1,
        'D' => head.y += 1,
        'L' => head.x -= 1,
        'R' => head.x += 1,
        _ => panic!(),
    }
}

fn move_follow(rope: &mut Vec<Coord>, i: usize) {
    let h = &rope[i - 1];
    let t = &rope[i];

    if is_touching(h, t) {
        return;
    }

    let r = match (h, t) {
        (h, t) if t.x == h.x && h.y < t.y => Coord { x: t.x, y: t.y - 1 },
        (h, t) if t.x < h.x && h.y < t.y => Coord {
            x: t.x + 1,
            y: t.y - 1,
        },
        (h, t) if t.x < h.x && h.y == t.y => Coord { x: t.x + 1, y: t.y },
        (h, t) if t.x < h.x && h.y > t.y => Coord {
            x: t.x + 1,
            y: t.y + 1,
        },
        (h, t) if t.x == h.x && h.y > t.y => Coord { x: t.x, y: t.y + 1 },
        (h, t) if t.x > h.x && h.y > t.y => Coord {
            x: t.x - 1,
            y: t.y + 1,
        },
        (h, t) if t.x > h.x && h.y == t.y => Coord { x: t.x - 1, y: t.y },
        (h, t) if t.x > h.x && h.y < t.y => Coord {
            x: t.x - 1,
            y: t.y - 1,
        },
        _ => panic!(),
    };

    rope[i] = r;
}

fn is_touching(c1: &Coord, c2: &Coord) -> bool {
    let dist_x = (c1.x - c2.x).abs();
    let dist_y = (c1.y - c2.y).abs();
    if dist_x >= 2 || dist_y >= 2 {
        false
    } else {
        true
    }
}
