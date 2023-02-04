use std::collections::HashMap;

use lazy_regex::regex_captures;

type Bitmask = i128;
type Valve = i32;

fn str_to_valve(s: &str) -> Valve {
    let mut chars = s.chars();
    let c1 = chars.next().unwrap() as i32;
    let c2 = chars.next().unwrap() as i32;
    c1 << 8 | c2
}

pub fn solve() -> (i64, i64) {
    let buf = include_bytes!("../inputs/input16.txt");
    let mut graph: HashMap<Valve, Vec<Valve>> = HashMap::new();
    let mut flows: HashMap<Valve, i64> = HashMap::new();
    let mut valves: Vec<String> = Vec::new();
    let mut indices: HashMap<String, Bitmask> = HashMap::new();

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let (_, valve, flow_rate, leads_to) = regex_captures!(
            r#"Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)"#,
            line
        )
        .unwrap();

        graph.insert(
            valve.to_string(),
            leads_to.split(',').map(|s| s.trim().to_string()).collect(),
        );
        valves.push(valve.to_string());

        let fr = flow_rate.parse::<i64>().unwrap();
        if fr > 0 {
            flows.insert(valve.to_string(), fr);
        }
    }

    valves.sort();

    for i in 0..valves.len() {
        indices.insert(valves[i].clone(), 1 << i);
    }

    // Distances between any pair of nodes (Floyd-Warshall)
    let mut dists: HashMap<(String, String), i64> = HashMap::new();
    for v in &valves {
        for l in &valves {
            let pair = (v.clone(), l.clone());
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
                let dist_ij = *dists.get(&(i.to_string(), j.to_string())).unwrap();
                let dist_ik = *dists.get(&(i.to_string(), k.to_string())).unwrap();
                let dist_kj = *dists.get(&(k.to_string(), j.to_string())).unwrap();
                dists.insert(
                    (i.to_string(), j.to_string()),
                    dist_ij.min(dist_ik + dist_kj),
                );
            }
        }
    }

    let mut answers_p1 = HashMap::new();
    let start: String = "AA".to_string();
    visit(&start, 30, 0, 0, &mut answers_p1, &flows, &dists, &indices);
    let p1 = *answers_p1.values().max().unwrap();

    let mut answers_p2 = HashMap::new();
    visit(&start, 26, 0, 0, &mut answers_p2, &flows, &dists, &indices);
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
    valve: &String,
    minutes: i64,
    bitmask: Bitmask,
    pressure: i64,
    answer: &mut HashMap<Bitmask, i64>,
    flows: &HashMap<String, i64>,
    dists: &HashMap<(String, String), i64>,
    indices: &HashMap<String, Bitmask>,
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
        let dist = dists.get(&(valve.to_string(), valve2.to_string())).unwrap();
        let remaining_minutes = minutes - dist - 1;
        if remaining_minutes > 0 {
            let i: Bitmask = *indices.get(valve2).unwrap();
            if bitmask & i == 0 {
                visit(
                    &valve2,
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
