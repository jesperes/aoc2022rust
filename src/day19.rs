use fnv::FnvHashMap;
use rayon::prelude::*;

use lazy_regex::regex_captures;

#[derive(Debug)]
struct Blueprint {
    nr: i32,
    ore_robot_ore_cost: i32,
    clay_robot_ore_cost: i32,
    obsidian_robot_ore_cost: i32,
    obsidian_robot_clay_cost: i32,
    geode_robot_ore_cost: i32,
    geode_robot_obsidian_cost: i32,

    max_ore_r: i32,
    max_clay_r: i32,
    max_obsidian_r: i32,
}

impl Blueprint {
    fn from_str(text: &str) -> Self {
        let (
            _,
            nr,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,
        ) = regex_captures!(
            "Blueprint (\\d+): Each ore robot costs (\\d+) ore. \
            Each clay robot costs (\\d+) ore. \
            Each obsidian robot costs (\\d+) ore and (\\d+) clay. \
            Each geode robot costs (\\d+) ore and (\\d+) obsidian.",
            text
        )
        .unwrap();

        let nr = nr.parse().unwrap();
        let ore_robot_ore_cost = ore_robot_ore_cost.parse().unwrap();
        let clay_robot_ore_cost = clay_robot_ore_cost.parse().unwrap();
        let obsidian_robot_ore_cost = obsidian_robot_ore_cost.parse().unwrap();
        let obsidian_robot_clay_cost = obsidian_robot_clay_cost.parse().unwrap();
        let geode_robot_ore_cost = geode_robot_ore_cost.parse().unwrap();
        let geode_robot_obsidian_cost = geode_robot_obsidian_cost.parse().unwrap();

        Blueprint {
            nr,
            ore_robot_ore_cost,
            clay_robot_ore_cost,
            obsidian_robot_ore_cost,
            obsidian_robot_clay_cost,
            geode_robot_ore_cost,
            geode_robot_obsidian_cost,

            max_ore_r: clay_robot_ore_cost
                .max(obsidian_robot_ore_cost)
                .max(geode_robot_ore_cost),
            max_clay_r: obsidian_robot_clay_cost,
            max_obsidian_r: geode_robot_obsidian_cost,
        }
    }
}

fn parse_blueprints(text: &[u8]) -> Vec<Blueprint> {
    String::from_utf8_lossy(text)
        .trim()
        .split("\n")
        .map(|line| Blueprint::from_str(line))
        .collect()
}

type CacheKey = (i32, i32, i32, i32, i32, i32, i32, i32, i32);

fn search2(
    bp: &Blueprint,
    cache: &mut FnvHashMap<CacheKey, i32>,
    minutes_left: i32,
    // Resource amounts
    ore: i32,
    clay: i32,
    obs: i32,
    geo: i32,
    // Number of robots
    ore_r: i32,
    clay_r: i32,
    obs_r: i32,
    geo_r: i32,
) -> i32 {
    if minutes_left == 0 {
        return geo;
    }

    let mut max_geodes: i32 = 0;
    let key = (
        minutes_left,
        ore,
        clay,
        obs,
        geo,
        ore_r,
        clay_r,
        obs_r,
        geo_r,
    );

    if let Some(cached_value) = cache.get(&key) {
        return *cached_value;
    }

    // Always build a geode robot, if possible, and do not investigate other branches
    if obs >= bp.geode_robot_obsidian_cost && ore >= bp.geode_robot_ore_cost {
        return search2(
            bp,
            cache,
            minutes_left - 1,
            ore - bp.geode_robot_ore_cost + ore_r,
            clay + clay_r,
            obs - bp.geode_robot_obsidian_cost + obs_r,
            geo + geo_r,
            ore_r,
            clay_r,
            obs_r,
            geo_r + 1,
        );
    }

    // Optimization 1. This is the last robot we can build, so if we couldn't
    // build a geode robot, there is really no point in building anything else,
    // but remember to let the geode robots we have build the last round of
    // robots. (This optimization took the total runtime from 49s to 36s.)
    if minutes_left == 1 {
        return geo + geo_r;
    }

    // Maybe build obsidian robot
    if ore >= bp.obsidian_robot_ore_cost
        && clay >= bp.obsidian_robot_clay_cost
        && obs_r < bp.max_obsidian_r
    {
        let geodes = search2(
            bp,
            cache,
            minutes_left - 1,
            ore - bp.obsidian_robot_ore_cost + ore_r,
            clay - bp.obsidian_robot_clay_cost + clay_r,
            obs + obs_r,
            geo + geo_r,
            ore_r,
            clay_r,
            obs_r + 1,
            geo_r,
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Optimization: skip if we don't have time to build a obsidian + a geode
    // robot. (22s -> 17.5s).
    if minutes_left == 2 {
        return geo + geo_r * 2;
    }

    // Maybe build clay robot
    if ore >= bp.clay_robot_ore_cost && clay_r < bp.max_clay_r {
        let geodes = search2(
            bp,
            cache,
            minutes_left - 1,
            ore - bp.clay_robot_ore_cost + ore_r,
            clay + clay_r,
            obs + obs_r,
            geo + geo_r,
            ore_r,
            clay_r + 1,
            obs_r,
            geo_r,
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Maybe build ore robot
    if ore >= bp.ore_robot_ore_cost && ore_r < bp.max_ore_r {
        let geodes = search2(
            bp,
            cache,
            minutes_left - 1,
            ore - bp.ore_robot_ore_cost + ore_r,
            clay + clay_r,
            obs + obs_r,
            geo + geo_r,
            ore_r + 1,
            clay_r,
            obs_r,
            geo_r,
        );

        if geodes > max_geodes {
            max_geodes = geodes;
        }
    }

    // Don't build anything, just let existing robots produce more resources
    let geodes = search2(
        bp,
        cache,
        minutes_left - 1,
        ore + ore_r,
        clay + clay_r,
        obs + obs_r,
        geo + geo_r,
        ore_r,
        clay_r,
        obs_r,
        geo_r,
    );

    if geodes > max_geodes {
        max_geodes = geodes;
    }

    cache.insert(key, max_geodes);
    return max_geodes;
}

fn search(bp: &Blueprint, minutes_left: i32) -> i32 {
    let mut cache = FnvHashMap::default();
    return search2(&bp, &mut cache, minutes_left, 0, 0, 0, 0, 1, 0, 0, 0);
}

pub fn solve() -> (i32, i32) {
    let input_bytes = include_bytes!("../inputs/input19.txt");
    let blueprints = parse_blueprints(input_bytes);

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
