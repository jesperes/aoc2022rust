use lazy_regex::regex_captures;
use rayon::prelude::*;

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

const SKIP_ORE: i32 = 0b001;
const SKIP_CLAY: i32 = 0b010;
const SKIP_OBS: i32 = 0b100;

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

fn dfs(
    bp: &Blueprint,
    global_max: &mut i32,
    skip_list: i32,
    min: i32,
    o: i32,
    c: i32,
    b: i32,
    g: i32,
    or: i32,
    cr: i32,
    br: i32,
    gr: i32,
) -> i32 {
    let ooc = bp.ore_robot_ore_c;
    let coc = bp.clay_robot_ore_c;
    let boc = bp.obs_robot_ore_c;
    let bcc = bp.obs_robot_clay_c;
    let goc = bp.geo_robot_ore_c;
    let gbc = bp.geo_robot_obs_c;

    let can_build_obs = c >= bcc && o >= boc && or < bp.max_obs_r;
    let can_build_clay = o >= coc && cr < bp.max_clay_r;
    let can_build_ore = o >= ooc && or < bp.max_ore_r;
    let can_build_geo = b >= gbc && o >= goc;

    let theoretical_max_geo = g + gr * min + (min * (min - 1)) << 1;

    if min == 1 {
        g + gr
    } else if theoretical_max_geo < *global_max {
        0
    } else if can_build_geo {
        dfs(
            bp,
            global_max,
            0,
            min - 1,
            o + or - goc,
            c + cr,
            b + br - gbc,
            g + gr,
            or,
            cr,
            br,
            gr + 1,
        )
    } else {
        let mut maxlist = vec![];
        let skip_obs = (skip_list & SKIP_OBS) != 0;
        let skip_clay = (skip_list & SKIP_CLAY) != 0;
        let skip_ore = (skip_list & SKIP_ORE) != 0;

        if can_build_obs && !skip_obs {
            maxlist.push(dfs(
                bp,
                global_max,
                0,
                min - 1,
                o + or - boc,
                c + cr - bcc,
                b + br,
                g + gr,
                or,
                cr,
                br + 1,
                gr,
            ))
        }

        if can_build_clay && !skip_clay {
            maxlist.push(dfs(
                bp,
                global_max,
                0,
                min - 1,
                o + or - coc,
                c + cr,
                b + br,
                g + gr,
                or,
                cr + 1,
                br,
                gr,
            ))
        }

        if can_build_ore && !skip_ore {
            maxlist.push(dfs(
                bp,
                global_max,
                0,
                min - 1,
                o + or - ooc,
                c + cr,
                b + br,
                g + gr,
                or + 1,
                cr,
                br,
                gr,
            ))
        }

        let mut new_skip_list = 0;
        if can_build_obs {
            new_skip_list |= SKIP_OBS;
        }
        if can_build_ore {
            new_skip_list |= SKIP_ORE;
        }
        if can_build_clay {
            new_skip_list |= SKIP_CLAY;
        }

        maxlist.push(dfs(
            bp,
            global_max,
            new_skip_list,
            min - 1,
            o + or,
            c + cr,
            b + br,
            g + gr,
            or,
            cr,
            br,
            gr,
        ));

        let new_max = maxlist.iter().max().unwrap();
        if new_max > global_max {
            *global_max = *new_max;
        }

        *new_max
    }
}

fn search(bp: &Blueprint, minutes_left: i32) -> i32 {
    let mut global_max = 0;
    dfs(
        &bp,
        &mut global_max,
        0,
        minutes_left,
        0,
        0,
        0,
        0,
        1,
        0,
        0,
        0,
    )
}

pub fn solve() -> (i32, i32) {
    let input_bytes = include_bytes!("../inputs/input19.txt");
    let blueprints = Blueprint::parse_blueprints(input_bytes);

    // Check all blueprints in parallel, then collect the results. Note that
    // we run all (both p1 and p2) blueprints in parallel.
    blueprints
        .iter()
        .map(|bp| (true, bp))
        .chain(
            blueprints
                .iter()
                .filter(|bp| bp.nr <= 3)
                .map(|bp| (false, bp)),
        )
        .par_bridge()
        .into_par_iter()
        .map(|(part, bp)| match part {
            true => (part, search(&bp, 24) * bp.nr),
            false => (part, search(&bp, 32)),
        })
        .collect::<Vec<(bool, i32)>>()
        .iter()
        .fold((0, 1), |(p1, p2), (part, sol)| match part {
            true => (p1 + sol, p2),
            false => (p1, p2 * sol),
        })
}
