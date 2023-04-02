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
}

pub fn solve() -> (i32, i32) {
    // let input_bytes = include_bytes!("../inputs/input18.txt");
    let input_bytes = "Blueprint 1: \
                       Each ore robot costs 4 ore. \
                       Each clay robot costs 2 ore. \
                       Each obsidian robot costs 3 ore and 14 clay. \
                       Each geode robot costs 2 ore and 7 obsidian."
        .as_bytes();
    let mut blueprints = Vec::new();

    for line in String::from_utf8_lossy(input_bytes).trim().split("\n") {
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
            line
        )
        .unwrap();

        blueprints.push(Blueprint {
            nr: nr.parse().unwrap(),
            ore_robot_ore_cost: ore_robot_ore_cost.parse().unwrap(),
            clay_robot_ore_cost: clay_robot_ore_cost.parse().unwrap(),
            obsidian_robot_ore_cost: obsidian_robot_ore_cost.parse().unwrap(),
            obsidian_robot_clay_cost: obsidian_robot_clay_cost.parse().unwrap(),
            geode_robot_ore_cost: geode_robot_ore_cost.parse().unwrap(),
            geode_robot_obsidian_cost: geode_robot_obsidian_cost.parse().unwrap(),
        });
    }

    for bp in &blueprints {
        let mut num_ore_robots = 1;
        let mut num_clay_robots = 0;
        let mut num_obsidian_robots = 0;
        let mut num_geode_robots = 0;
        let mut amount_of_ore = 0;
        let mut amount_of_clay = 0;
        let mut amount_of_obsidian = 0;
        let mut amount_of_geodes = 0;

        let max_ore = bp
            .clay_robot_ore_cost
            .max(bp.obsidian_robot_ore_cost)
            .max(bp.geode_robot_ore_cost);

        let max_clay = bp.obsidian_robot_clay_cost;

        let max_obsidian = bp.geode_robot_obsidian_cost;

        /*
           Simple heuristic:
           - Build a geode-, obsidian-, clay-, or ore-robot, in that order.
           - Do not build more robots than any robot costs, so if no robot
             costs more than 4 ore do not build more that 4 ore robots.

           TODO: rewrite this to a depth-first search, exploring all possible
           (valid) choices.
        */
        for minute in 1..=24 {
            println!("\n== Minute {minute} ==");
            let mut ore_robot_is_building = false;
            let mut clay_robot_is_building = false;
            let mut obsidian_robot_is_building = false;
            let mut geode_robot_is_building = false;

            // Build robots

            if amount_of_obsidian >= bp.geode_robot_obsidian_cost
                && amount_of_ore >= bp.geode_robot_ore_cost
            {
                geode_robot_is_building = true;
                amount_of_obsidian -= bp.geode_robot_obsidian_cost;
                amount_of_ore -= bp.geode_robot_ore_cost;
                println!(
                    "Spend {} obsidian and {} ore to start building a geode-cracking robot.",
                    bp.geode_robot_obsidian_cost, bp.geode_robot_ore_cost
                );
            }

            if amount_of_clay >= bp.obsidian_robot_clay_cost
                && amount_of_ore >= bp.obsidian_robot_ore_cost
                && amount_of_obsidian < max_obsidian
            {
                obsidian_robot_is_building = true;
                amount_of_clay -= bp.obsidian_robot_clay_cost;
                amount_of_ore -= bp.obsidian_robot_ore_cost;
                println!(
                    "Spend {} clay and {} obsidian to start building a obsidian-collecting robot.",
                    bp.obsidian_robot_clay_cost, bp.obsidian_robot_ore_cost
                );
            }

            if amount_of_ore >= bp.clay_robot_ore_cost && amount_of_clay < max_clay {
                clay_robot_is_building = true;
                amount_of_ore -= bp.clay_robot_ore_cost;
                println!(
                    "Spend {} ore to start building a clay-collecting robot.",
                    bp.clay_robot_ore_cost
                );
            }

            if amount_of_ore >= bp.ore_robot_ore_cost && amount_of_ore < max_ore {
                ore_robot_is_building = true;
                amount_of_ore -= bp.ore_robot_ore_cost;
                println!(
                    "Spend {} ore to start building a ore-collecting robot.",
                    bp.ore_robot_ore_cost
                );
            }

            // Collect ore from existing robots (but not from the ones which are building)

            amount_of_ore += num_ore_robots;
            if num_ore_robots >= 1 {
                println!("{num_ore_robots} ore-collecting robot(s) collects {num_ore_robots} ore; you now have {amount_of_ore} ore.");
            }

            amount_of_clay += num_clay_robots;
            if num_clay_robots >= 1 {
                println!("{num_clay_robots} clay-collecting robot(s) collects {num_clay_robots} clay; you now have {amount_of_clay} clay.");
            }

            amount_of_obsidian += num_obsidian_robots;
            if num_obsidian_robots >= 1 {
                println!("{num_obsidian_robots} obsidian-collecting robot(s) collects {num_obsidian_robots} obsidian; you now have {amount_of_obsidian} obsidian.");
            }

            amount_of_geodes += num_geode_robots;
            if num_geode_robots >= 1 {
                println!("{num_geode_robots} geode-cracking robot(s) cracks {num_geode_robots} geode; you now have {amount_of_geodes} of them.");
            }

            // Robots have finished building

            if geode_robot_is_building {
                num_geode_robots += 1;
                println!(
                    "The new geode-collecting robot is ready; you now have {num_geode_robots} of them"
                );
            }

            if obsidian_robot_is_building {
                num_obsidian_robots += 1;
                println!(
                    "The new obsidian-collecting robot is ready; you now have {num_obsidian_robots} of them"
                );
            }

            if clay_robot_is_building {
                num_clay_robots += 1;
                println!(
                    "The new clay-collecting robot is ready; you now have {num_clay_robots} of them"
                );
            }

            if ore_robot_is_building {
                num_ore_robots += 1;
                println!(
                    "The new ore-collecting robot is ready; you now have {num_ore_robots} of them"
                );
            }
        }

        println!(
            "Blueprint {} gave {} geodes in 24 minutes",
            bp.nr, amount_of_geodes,
        );
    }

    (0, 0)
}
