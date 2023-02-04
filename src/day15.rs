use lazy_regex::regex_captures;
use std::collections::{HashMap, HashSet};

// This actually needs to be i128, i64 is not enough
type IntType = i128;
type Coord = (IntType, IntType);
type Line = (Coord, Coord);
type Interval = (IntType, IntType);

struct SensorInfo {
    sensor: Coord,
    dist: IntType,
}

struct Input {
    sensor_infos: Vec<SensorInfo>,
    sensors: HashSet<Coord>,
    beacons: HashSet<Coord>,
}

fn parse(buf: &[u8]) -> Input {
    let mut input = Input {
        sensor_infos: Vec::new(),
        sensors: HashSet::new(),
        beacons: HashSet::new(),
    };

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let (_, sx, sy, bx, by) = regex_captures!(
            r#"Sensor at x=(.*), y=(.*): closest beacon is at x=(.*), y=(.*)"#,
            line
        )
        .unwrap();
        let sensor = (
            sx.parse::<IntType>().unwrap(),
            sy.parse::<IntType>().unwrap(),
        );
        let beacon = (
            bx.parse::<IntType>().unwrap(),
            by.parse::<IntType>().unwrap(),
        );
        input.beacons.insert(beacon);
        input.sensors.insert(sensor);
        input.sensor_infos.push(SensorInfo {
            sensor,
            dist: dist(sensor, beacon),
        });
    }

    input
}

fn dist(sensor: Coord, beacon: Coord) -> IntType {
    (sensor.0 - beacon.0).abs() + (sensor.1 - beacon.1).abs()
}

fn edge_lines(sensor: &Coord, d: IntType) -> Vec<Line> {
    let x = (*sensor).0;
    let y = (*sensor).1;
    vec![
        ((x, y - d), (x + d, y)),
        ((x + d, y), (x, y + d)),
        ((x, y + d), (x - d, y)),
        ((x - d, y), (x, y - d)),
    ]
}

fn perimeter_lines(sensor: &Coord, d: IntType) -> Vec<Line> {
    let x = (*sensor).0;
    let y = (*sensor).1;
    vec![
        ((x, y - d - 1), (x + d, y - 1)),
        ((x + d + 1, y), (x + 1, y + d)),
        ((x, y + d + 1), (x - d, y + 1)),
        ((x - d - 1, y), (x - 1, y - d)),
    ]
}

fn is_valid_x(x: IntType, line: Line) -> bool {
    let (a, b) = line;
    let (ax, _ay) = a;
    let (bx, _by) = b;

    if ax <= bx {
        x >= ax && x <= bx
    } else {
        is_valid_x(x, (b, a))
    }
}

fn is_valid_y(y: IntType, line: Line) -> bool {
    let (a, b) = line;
    let (_ax, ay) = a;
    let (_bx, by) = b;

    if ay <= by {
        y >= ay && y <= by
    } else {
        is_valid_y(y, (b, a))
    }
}

fn intersects(line1: Line, line2: Line) -> Option<Coord> {
    let (a, b) = line1;
    let (c, d) = line2;
    let (ax, ay) = a;
    let (bx, by) = b;
    let (cx, cy) = c;
    let (dx, dy) = d;
    let a1 = by - ay;
    let b1 = ax - bx;
    let c1 = a1 * ax + b1 * ay;
    let a2 = dy - cy;
    let b2 = cx - dx;
    let c2 = a2 * cx + b2 * cy;
    let det = a1 * b2 - a2 * b1;

    if det == 0 {
        None
    } else {
        let x0 = b2 * c1 - b1 * c2;
        let y0 = a1 * c2 - a2 * c1;

        if x0 % det == 0 && y0 % det == 0 {
            let x00 = x0.div_floor(det);
            let y00 = y0.div_floor(det);

            if is_valid_x(x00, line1)
                && is_valid_x(x00, line2)
                && is_valid_y(y00, line1)
                && is_valid_y(y00, line2)
            {
                Some((x00, y00))
            } else {
                None
            }
        } else {
            None
        }
    }
}

fn count_intervals(intervals: &mut Vec<Interval>) -> IntType {
    let n = intervals.len();
    let mut count = 0;
    let mut i = 0;

    while i < n {
        if i == n - 1 {
            let (x1, x2) = intervals[i];
            count += x2 - x1 + 1;
            break;
        } else {
            let i1 = intervals[i];
            let i2 = intervals[i + 1];
            let (x1, x2) = i1;
            let (x3, x4) = i2;

            if x4 <= x2 {
                intervals[i + 1] = intervals[i];
                i += 1;
            } else if x3 <= x2 {
                intervals[i + 1].0 = x1;
                i += 1;
            } else if x2 < x3 {
                count += x2 - x1 + 1;
                i += 1;
            } else {
                panic!()
            }
        }
    }

    count
}

fn part1(input: &Input) -> IntType {
    let x_min = input
        .sensor_infos
        .iter()
        .map(|si| si.sensor.0 - si.dist)
        .min()
        .unwrap();

    let x_max = input
        .sensor_infos
        .iter()
        .map(|si| si.sensor.0 + si.dist)
        .max()
        .unwrap();

    let y = 2000000;
    let line = ((x_min, y), (x_max, y));
    let mut intervals: Vec<Interval> = vec![];

    for si in &input.sensor_infos {
        let intersects: Vec<Coord> = edge_lines(&si.sensor, si.dist)
            .iter()
            .filter_map(|el| intersects(line, *el))
            .collect();

        match intersects[..] {
            [] => {}
            [_] => {}
            [(ax, _), (bx, _)] if ax <= bx => {
                intervals.push((ax, bx));
            }
            [(ax, _), (bx, _)] => {
                intervals.push((bx, ax));
            }
            _ => panic!(),
        }
    }

    intervals.sort();
    let count = count_intervals(&mut intervals);
    let mut items_on_y_line = 0;
    for si in input.sensors.union(&input.beacons) {
        if si.1 == y {
            items_on_y_line += 1;
        }
    }

    count - items_on_y_line
}

fn all_perimeter_lines(input: &Input) -> Vec<Line> {
    let mut pl: Vec<Line> = vec![];
    for si in &input.sensor_infos {
        pl.append(&mut perimeter_lines(&si.sensor, si.dist));
    }
    return pl;
}

fn part2(input: &Input) -> IntType {
    let pl = all_perimeter_lines(input);
    let mut pairs: Vec<(Line, Line)> = vec![];
    let mut freq: HashMap<Coord, IntType> = HashMap::new();

    for l1 in &pl {
        for l2 in &pl {
            if l1 < l2 {
                let pair = (*l1, *l2);
                pairs.push(pair);
            }
        }
    }

    for (line1, line2) in pairs {
        match intersects(line1, line2) {
            None => {}
            Some(pos) => {
                if is_in_range_of_any_sensor(pos, &input.sensor_infos) {
                    freq.remove(&pos);
                } else {
                    *freq.entry(pos).or_insert(0) += 1;
                }
            }
        }
    }

    let (pos, _) = freq.iter().filter(|(_, v)| **v >= 4).next().unwrap();
    let (x, y) = pos;
    x * 4000000 + y
}

fn is_in_range_of_any_sensor(pos: (i128, i128), sensor_infos: &[SensorInfo]) -> bool {
    for si in sensor_infos {
        if dist(pos, si.sensor) <= si.dist {
            return true;
        }
    }
    return false;
}

pub fn solve() -> (IntType, IntType) {
    let buf = include_bytes!("../inputs/input15.txt");
    let input = parse(buf);
    let p1 = part1(&input);
    let p2 = part2(&input);
    (p1, p2)
}
