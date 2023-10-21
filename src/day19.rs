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
struct Inv {
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

impl Inv {
    fn default(min: i32) -> Inv {
        Inv {
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
            max_clay_r: obs_robot_clay_c,
            max_obs_r: geo_robot_obs_c,
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

fn search2(bp: &Blueprint, cache: &mut FnvHashMap<Inv, i32>, inv: Inv) -> i32 {
    if inv.min == 0 {
        return inv.geo;
    }

    let mut max_geodes: i32 = 0;

    if let Some(cached_value) = cache.get(&inv) {
        return *cached_value;
    }

    // Default inventory for the next minute
    let next_inv = Inv {
        min: inv.min - 1,
        ore: inv.ore + inv.ore_r,
        clay: inv.clay + inv.clay_r,
        obs: inv.obs + inv.obs_r,
        geo: inv.geo + inv.geo_r,
        ..inv
    };

    // Always build a geode robot, if possible, and do not investigate other branches
    if inv.obs >= bp.geo_robot_obs_c && inv.ore >= bp.geo_robot_ore_c {
        return search2(
            bp,
            cache,
            Inv {
                ore: next_inv.ore - bp.geo_robot_ore_c,
                obs: next_inv.obs - bp.geo_robot_obs_c,
                geo_r: inv.geo_r + 1,
                ..next_inv
            },
        );
    }

    // Optimization 1. This is the last robot we can build, so if we couldn't
    // build a geode robot, there is really no point in building anything else,
    // but remember to let the geode robots we have build the last round of
    // robots. (This optimization took the total runtime from 49s to 36s.)
    if inv.min == 1 {
        return inv.geo + inv.geo_r;
    }

    // Maybe build obsidian robot
    if inv.ore >= bp.obs_robot_ore_c && inv.clay >= bp.obs_robot_clay_c && inv.obs_r < bp.max_obs_r
    {
        let geodes = search2(
            bp,
            cache,
            Inv {
                ore: next_inv.ore - bp.obs_robot_ore_c,
                clay: next_inv.clay - bp.obs_robot_clay_c,
                obs_r: next_inv.obs_r + 1,
                ..next_inv
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Optimization: skip if we don't have time to build a obsidian + a geode
    // robot. (22s -> 17.5s).
    if inv.min == 2 {
        return inv.geo + inv.geo_r * 2;
    }

    // Maybe build clay robot
    if inv.ore >= bp.clay_robot_ore_c && inv.clay_r < bp.max_clay_r {
        let geodes = search2(
            bp,
            cache,
            Inv {
                ore: next_inv.ore - bp.clay_robot_ore_c,
                clay_r: next_inv.clay_r + 1,
                ..next_inv
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Maybe build ore robot
    if inv.ore >= bp.ore_robot_ore_c && inv.ore_r < bp.max_ore_r {
        let geodes = search2(
            bp,
            cache,
            Inv {
                ore: next_inv.ore - bp.ore_robot_ore_c,
                ore_r: next_inv.ore_r + 1,
                ..next_inv
            },
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Don't build anything, just let existing robots produce more resources
    let geodes = search2(bp, cache, next_inv);

    if geodes > max_geodes {
        max_geodes = geodes;
    }

    cache.insert(inv, max_geodes);
    return max_geodes;
}

fn search(bp: &Blueprint, minutes_left: i32) -> i32 {
    let mut cache = FnvHashMap::default();
    return search2(&bp, &mut cache, Inv::default(minutes_left));
}

pub fn solve() -> (i32, i32) {
    let input_bytes = include_bytes!("../inputs/input19.txt");
    let blueprints = Blueprint::parse_blueprints(input_bytes);

    // Optimization 2: parallelization brought the runtime from 36s to 20s.

    let p1 = blueprints
        .par_iter()
        .map(|bp| search(&bp, 24) * bp.nr)
        .sum();

    let p2 = blueprints[0..3]
        .par_iter()
        .map(|bp| search(&bp, 32))
        .reduce(|| 1, |a, b| a * b);

    (p1, p2)
}
