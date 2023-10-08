use std::collections::HashSet;

use itertools::Itertools;
use queues::*;

type Cube = (i32, i32, i32);

fn adjacent(c1: &Cube, c2: &Cube) -> bool {
    let (a, b, c) = c1;
    let (d, e, f) = c2;
    (a - d).abs() + (b - e).abs() + (c - f).abs() == 1
}

fn maybe_enqueue(c: &Cube, queue: &mut Queue<Cube>) {
    let (x, y, z) = c;
    if *x < -1 || *x > 20 || *y < -1 || *y > 20 || *y < -1 || *z > 20 {
        // out of bounds
        return;
    }

    queue.add(*c).unwrap();
}

fn fill(coords: &Vec<Cube>) -> i32 {
    let lava: HashSet<Cube> = HashSet::from_iter(coords.iter().cloned());
    let mut area = 0;
    let mut water: HashSet<Cube> = HashSet::new();
    let mut queue: Queue<Cube> = queue![(0, 0, 0)];

    while let Ok(c) = queue.remove() {
        let (x, y, z) = c;
        if x < -1 || x > 20 || y < -1 || y > 20 || z < -1 || z > 20 {
            continue;
        } else if lava.contains(&c) {
            area += 1;
        } else if water.contains(&c) {
            continue;
        } else {
            water.insert(c);
            maybe_enqueue(&(x + 1, y, z), &mut queue);
            maybe_enqueue(&(x - 1, y, z), &mut queue);
            maybe_enqueue(&(x, y + 1, z), &mut queue);
            maybe_enqueue(&(x, y - 1, z), &mut queue);
            maybe_enqueue(&(x, y, z + 1), &mut queue);
            maybe_enqueue(&(x, y, z - 1), &mut queue);
        }
    }

    area
}

pub fn solve() -> (usize, i32) {
    let bytes = include_bytes!("../inputs/input18.txt");
    let s = String::from_utf8_lossy(bytes);

    let coords: Vec<Cube> = s
        .trim()
        .split("\n")
        .map(|line| {
            line.split(",")
                .into_iter()
                .map(|s| s.parse().unwrap())
                .next_tuple::<Cube>()
                .unwrap()
        })
        .collect();

    let mut total_surface_area = coords.len() * 6;

    for c1 in &coords {
        for c2 in &coords {
            if c1 < c2 {
                if adjacent(&c1, &c2) {
                    total_surface_area -= 2;
                }
            }
        }
    }

    let area = fill(&coords);

    (total_surface_area, area)
}
