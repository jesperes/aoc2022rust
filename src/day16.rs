use fnv::FnvHashMap; // faster hashmap
use grid::Grid;
use lazy_regex::regex_captures;

type Bitmask = i64;

// Represent a valve as an integer 0..(26*26)
// type Valve = i32;
/// type ValvePair = i32;

// // Map valve names to integers
// fn str_to_valve(s: &str) -> Valve {
//     let mut chars = s.chars();
//     let base = 'A' as i32;
//     let n1 = chars.next().unwrap() as i32 - base;
//     let n2 = chars.next().unwrap() as i32 - base;
//     (n1 * 26 + n2) as Valve
// }

// fn valve_mask(valve: &Valve) -> Bitmask {
//     1 << valve
// }

type Flows = FnvHashMap<usize, i64>;
type Indices = Vec<Bitmask>;
type Graph = Vec<Vec<usize>>;
type DistGrid = Grid<i64>;
type AnswerMap = FnvHashMap<Bitmask, i64>;

pub fn solve() -> (i64, i64) {
    const NUM_LETTERS: usize = 26;
    const NUM_VALVES: usize = NUM_LETTERS * NUM_LETTERS;

    // Keep a map from valve name to an integer. The only valve name we actually
    // care about is "AA" which is the start valve. All others can just be mapped to
    // an arbitrary integer.
    let mut idx: usize = 0;
    let mut valve_map: FnvHashMap<String, usize> = FnvHashMap::default();
    let mut start: usize = 0;

    let buf = include_bytes!("../inputs/input16.txt");
    let mut graph_pre: Vec<String> = Vec::new();
    let mut graph: Graph = Vec::new();
    let mut flows: Flows = FnvHashMap::default();
    let mut indices: Indices = vec![];
    let mut dists: DistGrid = Grid::new(NUM_VALVES, NUM_VALVES);

    for line in String::from_utf8_lossy(buf).trim().split("\n") {
        let (_, valve, flow_rate, leads_to) = regex_captures!(
            r#"Valve (.*) has flow rate=(.*); tunnels? leads? to valves? (.*)"#,
            line
        )
        .unwrap();

        valve_map.insert(valve.to_string(), idx);
        if valve == "AA" {
            start = idx;
        }

        indices.push(1 << idx);

        let fr = flow_rate.parse::<i64>().unwrap();
        if fr > 0 {
            flows.insert(idx, fr);
        }

        graph_pre.push(leads_to.to_string());
        idx += 1;
    }

    let num_valves = idx;

    // Build the graph now that all valves have been mapped to integers
    for i in 0..num_valves {
        graph.push(
            graph_pre[i]
                .split(',')
                .map(|s| *valve_map.get(s.to_string().trim()).unwrap())
                .collect::<Vec<usize>>(),
        );
    }

    // Distances between any pair of nodes (Floyd-Warshall)
    for v in 0..num_valves {
        for l in 0..num_valves {
            if graph[v].contains(&l) {
                dists[v][l] = 1;
            } else {
                dists[v][l] = 10000;
            }
        }
    }

    for k in 0..num_valves {
        for i in 0..num_valves {
            for j in 0..num_valves {
                let dist_ij = dists[i][j];
                let dist_ik = dists[i][k];
                let dist_kj = dists[k][j];
                dists[i][j] = dist_ij.min(dist_ik + dist_kj);
            }
        }
    }

    let mut answers_p1 = FnvHashMap::default();
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
    valve: usize,
    minutes: i64,
    bitmask: Bitmask,
    pressure: i64,
    answer: &mut AnswerMap,
    flows: &Flows,
    dists: &DistGrid,
    indices: &Indices,
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
        let dist = dists[valve][*valve2];
        let remaining_minutes = minutes - dist - 1;
        if remaining_minutes > 0 {
            let i: Bitmask = indices[*valve2];
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
