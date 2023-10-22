use fnv::FnvHashMap;
use rayon::prelude::*;

use lazy_regex::regex_captures;

#[derive(Debug)]
struct Blueprint {
    nr: i32,
    ore_robot_ore_c: i32,
    clay_robot_ore_c: i32,
    obs_robot_ore_c: i32,
    obs_robot_clay_c: i32,
    geo_robot_ore_c: i32,
    geo_robot_obs_c: i32,

    max_ore_r: i32,
    max_clay_r: i32,
    max_obs_r: i32,
}

#[derive(Eq, Hash, PartialEq)]
struct State {
    min: i32,
    ore: i32,
    clay: i32,
    obs: i32,
    geo: i32,
    ore_r: i32,
    clay_r: i32,
    obs_r: i32,
    geo_r: i32,
}

impl State {
    fn default(min: i32) -> State {
        State {
            min,
            ore: 0,
            clay: 0,
            obs: 0,
            geo: 0,
            ore_r: 1,
            clay_r: 0,
            obs_r: 0,
            geo_r: 0,
        }
    }
}

impl Blueprint {
    fn from_str(text: &str) -> Self {
        let (
            _,
            nr,
            ore_robot_ore_c,
            clay_robot_ore_c,
            obs_robot_ore_c,
            obs_robot_clay_c,
            geo_robot_ore_c,
            geo_robot_obs_c,
        ) = regex_captures!(
            "Blueprint (\\d+): Each ore robot costs (\\d+) ore. \
            Each clay robot costs (\\d+) ore. \
            Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
            Each geode robot costs (\\d+) ore and (\\d+) obsidian.",
            text
        )
        .unwrap();

        let nr = nr.parse().unwrap();
        let ore_robot_ore_c = ore_robot_ore_c.parse().unwrap();
        let clay_robot_ore_c = clay_robot_ore_c.parse().unwrap();
        let obs_robot_ore_c = obs_robot_ore_c.parse().unwrap();
        let obs_robot_clay_c = obs_robot_clay_c.parse().unwrap();
        let geo_robot_ore_c = geo_robot_ore_c.parse().unwrap();
        let geo_robot_obs_c = geo_robot_obs_c.parse().unwrap();

        Blueprint {
            nr,
            ore_robot_ore_c,
            clay_robot_ore_c,
            obs_robot_ore_c,
            obs_robot_clay_c,
            geo_robot_ore_c,
            geo_robot_obs_c,

            max_ore_r: clay_robot_ore_c.max(obs_robot_ore_c).max(geo_robot_ore_c),
            // It turns out that we can lower the max_clay_r and max_obs_r a
            // little bit without actually losing any optimal solutions.
            max_clay_r: obs_robot_clay_c - 2,
            max_obs_r: geo_robot_obs_c - 1,
        }
    }

    fn parse_blueprints(text: &[u8]) -> Vec<Blueprint> {
        String::from_utf8_lossy(text)
            .trim()
            .split("\n")
            .map(|line| Blueprint::from_str(line))
            .collect()
    }
}

fn search2(
    bp: &Blueprint,
    cache: &mut FnvHashMap<State, i32>,
    num_states: &mut i32,
    cutoff: i32,
    state: State,
) -> i32 {
    *num_states += 1;

    if state.min == 0 {
        return state.geo;
    }

    // If we haven't reached the max robot limits by the cutuff time, there is
    // no point in proceeding. The cutoff time has been determined through
    // observation. :)
    if state.min < cutoff
        && state.obs_r < bp.max_obs_r
        && state.clay_r < bp.max_clay_r
        && state.ore_r < bp.max_ore_r
    {
        return 0;
    }

    let mut max_geodes: i32 = 0;

    if let Some(cached_value) = cache.get(&state) {
        return *cached_value;
    }

    // Default inventory for the next minute
    let next_state = State {
        min: state.min - 1,
        ore: state.ore + state.ore_r,
        clay: state.clay + state.clay_r,
        obs: state.obs + state.obs_r,
        geo: state.geo + state.geo_r,
        ..state
    };

    // Always build a geode robot, if possible, and do not investigate other branches
    if state.obs >= bp.geo_robot_obs_c && state.ore >= bp.geo_robot_ore_c {
        return search2(
            bp,
            cache,
            num_states,
            cutoff,
            State {
                ore: next_state.ore - bp.geo_robot_ore_c,
                obs: next_state.obs - bp.geo_robot_obs_c,
                geo_r: state.geo_r + 1,
                ..next_state
            },
        );
    }

    if state.min == 1 {
        return state.geo + state.geo_r;
    }

    // Maybe build obsidian robot
    if state.ore >= bp.obs_robot_ore_c
        && state.clay >= bp.obs_robot_clay_c
        && state.obs_r < bp.max_obs_r
    {
        let geodes = search2(
            bp,
            cache,
            num_states,
            cutoff,
            State {
                ore: next_state.ore - bp.obs_robot_ore_c,
                clay: next_state.clay - bp.obs_robot_clay_c,
                obs_r: next_state.obs_r + 1,
                ..next_state
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    if state.min == 2 {
        return state.geo + state.geo_r * 2;
    }

    // Maybe build clay robot
    if state.ore >= bp.clay_robot_ore_c && state.clay_r < bp.max_clay_r {
        let geodes = search2(
            bp,
            cache,
            num_states,
            cutoff,
            State {
                ore: next_state.ore - bp.clay_robot_ore_c,
                clay_r: next_state.clay_r + 1,
                ..next_state
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Maybe build ore robot
    if state.ore >= bp.ore_robot_ore_c && state.ore_r < bp.max_ore_r {
        let geodes = search2(
            bp,
            cache,
            num_states,
            cutoff,
            State {
                ore: next_state.ore - bp.ore_robot_ore_c,
                ore_r: next_state.ore_r + 1,
                ..next_state
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Don't build anything, just let existing robots produce more resources
    let geodes = search2(bp, cache, num_states, cutoff, next_state);

    if geodes > max_geodes {
        max_geodes = geodes;
    }

    cache.insert(state, max_geodes);
    return max_geodes;
}

fn search(bp: &Blueprint, cutoff: i32, minutes_left: i32) -> i32 {
    let mut cache = FnvHashMap::default();
    let mut num_states = 0;
    let n = search2(
        &bp,
        &mut cache,
        &mut num_states,
        cutoff,
        State::default(minutes_left),
    );

    return n;
}

pub fn solve() -> (i32, i32) {
    let input_bytes = include_bytes!("../inputs/input19.txt");
    let blueprints = Blueprint::parse_blueprints(input_bytes);

    // Cutoff numbers found by trial-and-error... :)
    let cutoff_p1 = 2;
    let cutoff_p2 = 18;

    // Check all blueprints in parallel, then collect the results
    blueprints
        .iter()
        .map(|bp| (true, cutoff_p1, bp))
        .chain(
            blueprints
                .iter()
                .filter(|bp| bp.nr <= 3)
                .map(|bp| (false, cutoff_p2, bp)),
        )
        .par_bridge()
        .into_par_iter()
        .map(|(part, cutoff, bp)| match part {
            true => (part, search(&bp, cutoff, 24) * bp.nr),
            false => (part, search(&bp, cutoff, 32)),
        })
        .collect::<Vec<(bool, i32)>>()
        .iter()
        .fold((0, 1), |(p1, p2), (part, sol)| match part {
            true => (p1 + sol, p2),
            false => (p1, p2 * sol),
        })
}
