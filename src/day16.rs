use fnv::FnvHashMap; // faster hashmap
use lazy_regex::regex_captures;

type Bitmask = i128;
type Valve = i32;
type ValvePair = i32;

// Map valve names to integers
fn str_to_valve(s: &str) -> Valve {
    let mut chars = s.chars();
    let c1 = chars.next().unwrap() as i32;
    let c2 = chars.next().unwrap() as i32;
    c1 << 8 | c2
}

fn valve_pair(v1: &Valve, v2: &Valve) -> ValvePair {
    v1 << 16 | v2
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input16.txt");
    let mut graph: FnvHashMap<Valve, Vec<Valve>> = FnvHashMap::default();
    let mut flows: FnvHashMap<Valve, i64> = FnvHashMap::default();
    let mut valves: Vec<Valve> = Vec::new();
    let mut indices: FnvHashMap<Valve, Bitmask> = FnvHashMap::default();
    let mut dists: FnvHashMap<ValvePair, i64> = FnvHashMap::default();

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let (_, valve, flow_rate, leads_to) = regex_captures!(
            r#"Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)"#,
            line
        )
        .unwrap();

        graph.insert(
            str_to_valve(valve),
            leads_to
                .split(',')
                .map(|s| str_to_valve(s.trim()))
                .collect(),
        );

        valves.push(str_to_valve(valve));

        let fr = flow_rate.parse::<i64>().unwrap();
        if fr > 0 {
            flows.insert(str_to_valve(valve), fr);
        }
    }

    valves.sort();

    for i in 0..valves.len() {
        indices.insert(valves[i].clone(), 1 << i);
    }

    // Distances between any pair of nodes (Floyd-Warshall)
    for v in &valves {
        for l in &valves {
            let pair = valve_pair(v, l);
            if graph.get(v).unwrap().contains(l) {
                dists.insert(pair, 1);
            } else {
                dists.insert(pair, 10000);
            }
        }
    }

    for k in &valves {
        for i in &valves {
            for j in &valves {
                let dist_ij = *dists.get(&valve_pair(i, j)).unwrap();
                let dist_ik = *dists.get(&valve_pair(i, k)).unwrap();
                let dist_kj = *dists.get(&valve_pair(k, j)).unwrap();
                dists.insert(valve_pair(i, j), dist_ij.min(dist_ik + dist_kj));
            }
        }
    }

    let mut answers_p1 = FnvHashMap::default();
    let start = str_to_valve("AA");
    visit(start, 30, 0, 0, &mut answers_p1, &flows, &dists, &indices);
    let p1 = *answers_p1.values().max().unwrap();

    let mut answers_p2 = FnvHashMap::default();
    visit(start, 26, 0, 0, &mut answers_p2, &flows, &dists, &indices);
    let mut p2 = 0;
    for (k1, v1) in &answers_p2 {
        for (k2, v2) in &answers_p2 {
            if k1 & k2 == 0 {
                let maxflow = v1 + v2;
                if maxflow > p2 {
                    p2 = maxflow;
                }
            }
        }
    }

    (p1, p2)
}
fn visit(
    valve: Valve,
    minutes: i64,
    bitmask: Bitmask,
    pressure: i64,
    answer: &mut FnvHashMap<Bitmask, i64>,
    flows: &FnvHashMap<Valve, i64>,
    dists: &FnvHashMap<ValvePair, i64>,
    indices: &FnvHashMap<Valve, Bitmask>,
) {
    answer
        .entry(bitmask)
        .and_modify(|old| {
            if pressure > *old {
                *old = pressure
            }
        })
        .or_insert(pressure);

    for (valve2, flow) in flows {
        let dist = dists.get(&valve_pair(&valve, valve2)).unwrap();
        let remaining_minutes = minutes - dist - 1;
        if remaining_minutes > 0 {
            let i: Bitmask = *indices.get(valve2).unwrap();
            if bitmask & i == 0 {
                visit(
                    *valve2,
                    remaining_minutes,
                    bitmask | i,
                    pressure + flow * remaining_minutes,
                    answer,
                    flows,
                    dists,
                    indices,
                );
            }
        }
    }
}
