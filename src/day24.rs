use std::{collections::BTreeSet, fmt};

use hashbrown::{HashMap, HashSet};
use lazy_regex::regex;
use regex::Match;

#[derive(Debug, Eq, PartialEq, Hash, Copy, Clone, Ord, PartialOrd)]
struct Coord {
    x: i32,
    y: i32,
}
type GridMap = HashMap<Coord, char>;
type CoordSet = HashSet<Coord>;

impl fmt::Display for Coord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

// A* search node
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Node {
    fscore: i32,
    pos: Pos,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.fscore, self.pos)
    }
}

#[derive(Debug, Ord, PartialOrd, Eq, PartialEq, Hash, Clone)]
struct Pos {
    coord: Coord,
    time: i32,
}

impl fmt::Display for Pos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({},{})", self.coord, self.time)
    }
}

fn match_to_pair(m: &Match, width: i32) -> (Coord, char) {
    let coord = match_to_coord(m, width);
    let c = m.as_str().chars().next().unwrap();
    (coord, c)
}

fn match_to_coord(m: &Match, width: i32) -> Coord {
    let start = m.start() as i32;
    let x = m.start() as i32 % (width + 3);
    let y = start.div_floor(width + 3);
    Coord { x, y }
}

pub fn solve() -> (i32, i32) {
    let bytes = include_bytes!("../inputs/input24.txt");
    let input = String::from_utf8_lossy(bytes);
    let w = (input.find('\n').unwrap() + 1) as i32 - 3;
    let h = regex!(r"\n").find_iter(&input).count() as i32 - 2;
    let mut cache: HashMap<i32, CoordSet> = HashMap::new();

    let blizzards: GridMap = regex!(r"([<>v^])")
        .find_iter(&input)
        .map(|m| match_to_pair(&m, w))
        .collect::<HashMap<Coord, char>>();

    let walls = regex!(r"#")
        .find_iter(&input)
        .map(|m| match_to_coord(&m, w))
        .collect::<CoordSet>();

    let start = Coord { x: 1, y: 0 };
    let end = Coord { x: w, y: h + 1 };

    let p1 = search(start, end, &blizzards, &mut cache, &walls, w, h, 0);
    let p2a = search(end, start, &blizzards, &mut cache, &walls, w, h, p1);
    let p2b = search(start, end, &blizzards, &mut cache, &walls, w, h, p2a);
    (p1, p2b)
}

fn search(
    start: Coord,
    end: Coord,
    orig_blizzards: &GridMap,
    blizzard_cache: &mut HashMap<i32, CoordSet>,
    walls: &CoordSet,
    width: i32,
    height: i32,
    t0: i32,
) -> i32 {
    let mut open_set: BTreeSet<Node> = BTreeSet::new();
    let mut gscore: HashMap<Pos, i32> = HashMap::new();
    let start_pos = Pos {
        coord: start,
        time: t0,
    };

    gscore.insert(start_pos.clone(), 0);
    open_set.insert(Node {
        fscore: dist(&start, &end),
        pos: start_pos.clone(),
    });

    loop {
        let node = open_set.pop_first().unwrap(); // empty means search exhausted and failed

        if node.pos.coord == end {
            return node.pos.time - 1;
        } else {
            // Get neighbors of node, and cache any additional computed blizzard states
            let blizzards = blizzard_cache.entry(node.pos.time).or_insert_with(|| {
                orig_blizzards
                    .iter()
                    .map(|(coord, dir)| blizzard_pos(&coord, node.pos.time, *dir, width, height))
                    .collect::<CoordSet>()
            });

            let neighbors = all_neighbors(&node, width, height, &blizzards, &walls);
            for nbr in neighbors {
                // println!("Neighbor: {}", nbr);
                let tentative_gscore: i32 = *gscore.get(&node.pos).unwrap() + 1;
                let gscore_nbr: i32 = *gscore.get(&nbr).unwrap_or(&i32::MAX);

                if tentative_gscore < gscore_nbr {
                    // We have reached this position the first time, or through
                    // a shorter path than before
                    let nbr_node = Node {
                        fscore: tentative_gscore + dist(&nbr.coord, &end),
                        pos: nbr.clone(),
                    };
                    open_set.insert(nbr_node);
                    gscore.insert(nbr.clone(), tentative_gscore);
                }
            }
        }
    }
}

fn dist(a: &Coord, b: &Coord) -> i32 {
    (a.x - b.x).abs() + (a.y - b.y).abs()
}

fn all_neighbors(node: &Node, w: i32, h: i32, blizzards: &CoordSet, walls: &CoordSet) -> Vec<Pos> {
    let x = node.pos.coord.x;
    let y = node.pos.coord.y;
    vec![
        Coord { x: x + 1, y },
        Coord { x: x - 1, y },
        Coord { x, y: y + 1 },
        Coord { x, y: y - 1 },
        node.pos.coord,
    ]
    .iter()
    .filter_map(|coord| {
        let x = coord.x;
        let y = coord.y;
        if x >= 0
            && y >= 0
            && x < w + 2
            && y < h + 2
            && !blizzards.contains(coord)
            && !walls.contains(coord)
        {
            Some(Pos {
                coord: *coord,
                time: node.pos.time + 1,
            })
        } else {
            // println!("Not eligible neighbor: {:?}", coord);
            None
        }
    })
    .collect::<Vec<Pos>>()
}

fn blizzard_pos(coord: &Coord, time: i32, dir: char, w: i32, h: i32) -> Coord {
    let n = time;
    let x = coord.x;
    let y = coord.y;
    match dir {
        '<' => Coord {
            x: (((x - 1) + w - (n % w)) % w) + 1,
            y,
        },
        '>' => Coord {
            x: (((x - 1) + (n % w)) % w) + 1,
            y,
        },
        'v' => Coord {
            x,
            y: (((y - 1) + (n % h)) % h) + 1,
        },
        '^' => Coord {
            x,
            y: (((y - 1) + h - (n % h)) % h) + 1,
        },
        _ => unreachable!(),
    }
}
