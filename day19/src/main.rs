use lazy_static::lazy_static;
use regex::Regex;

const MINUTES: u64 = 32;

#[derive(Debug, Clone, Copy)]
struct Blueprint {
    id: u64,
    ore_robot_ore_cost: u64,
    clay_robot_ore_cost: u64,
    obsidian_robot_ore_cost: u64,
    obsidian_robot_clay_cost: u64,
    geode_robot_ore_cost: u64,
    geode_robot_obsidian_cost: u64,
}

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_passed: u64,
    saved_ore: u64,
    saved_clay: u64,
    saved_obsidian: u64,
    saved_geode: u64,
    ore_robots: u64,
    clay_robots: u64,
    obsidian_robots: u64,
    geode_robots: u64,
}

impl From<&str> for Blueprint {
    fn from(line: &str) -> Self {
        let captures = RE.captures(line).expect("Invalid input line");

        Blueprint {
            id: captures[1].parse().unwrap(),
            ore_robot_ore_cost: captures[2].parse().unwrap(),
            clay_robot_ore_cost: captures[3].parse().unwrap(),
            obsidian_robot_ore_cost: captures[4].parse().unwrap(),
            obsidian_robot_clay_cost: captures[5].parse().unwrap(),
            geode_robot_ore_cost: captures[6].parse().unwrap(),
            geode_robot_obsidian_cost: captures[7].parse().unwrap(),
        }
    }
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Blueprint (\d+): Each ore robot costs (\d+) ore. Each clay robot costs (\d+) ore. Each obsidian robot costs (\d+) ore and (\d+) clay. Each geode robot costs (\d+) ore and (\d+) obsidian.").unwrap();
}

static MAX_FROM_NEW_ROBOTS: [u64; 32] = [
    0, 1, 4, 10, 20, 35, 56, 84, 120, 165, 220, 286, 364, 455, 560, 680, 816, 969, 1140, 1330,
    1540, 1771, 2024, 2300, 2600, 2925, 3276, 3654, 4060, 4495, 4960, 5456,
];

fn main() {
    let blueprints: Vec<_> = aoc::input_lines()
        .map(|line| Blueprint::from(line.as_str()))
        .take(3)
        .collect();
    for blueprint in &blueprints {
        println!("{blueprint:?}")
    }

    for i in 0..MINUTES {
        println!("{i}: {}", i * (i + 1) * (i + 2) / 6);
    }

    let weighted_sum: u64 = blueprints
        .iter()
        .map(|blueprint| {
            let first_state = State {
                minutes_passed: 0,
                saved_ore: 0,
                saved_clay: 0,
                saved_obsidian: 0,
                saved_geode: 0,
                ore_robots: 1,
                clay_robots: 0,
                obsidian_robots: 0,
                geode_robots: 0,
            };
            let mut best_found = 0;
            let max_geodes = seek(&first_state, blueprint, &mut best_found);
            println!("Blueprint {}, max geodes: {max_geodes}", blueprint.id);
            max_geodes * blueprint.id
        })
        .sum();
    println!("Part 1: {weighted_sum}");
}

fn seek(state: &State, blueprint: &Blueprint, best_found: &mut u64) -> u64 {
    if state.minutes_passed == MINUTES {
        return state.saved_geode;
    }

    if state.minutes_passed <= 8 {
        println!(
            "Blueprint #{}, Best found: {best_found}, {state:?}",
            blueprint.id
        );
    }

    // For new builds
    let robot_time_remaining = MINUTES - state.minutes_passed - 1;

    if *best_found
        >= state.saved_geode
            + MINUTES * state.geode_robots
            + MAX_FROM_NEW_ROBOTS[robot_time_remaining as usize]
    {
        //println!("Pruned branch. {robot_time_remaining} {max_from_new_robots}");
        return 0;
    }

    let round_start_ore = state.saved_ore;
    let round_start_clay = state.saved_clay;
    let round_start_obsidian = state.saved_obsidian;

    let new_state = State {
        minutes_passed: state.minutes_passed + 1,
        saved_ore: state.saved_ore + state.ore_robots,
        saved_clay: state.saved_clay + state.clay_robots,
        saved_obsidian: state.saved_obsidian + state.obsidian_robots,
        saved_geode: state.saved_geode + state.geode_robots,
        ore_robots: state.ore_robots,
        clay_robots: state.clay_robots,
        obsidian_robots: state.obsidian_robots,
        geode_robots: state.geode_robots,
    };

    let mut max_geodes: u64 = seek(&new_state, blueprint, best_found);
    *best_found = max_geodes.max(*best_found);

    if round_start_ore >= blueprint.ore_robot_ore_cost {
        let mut build_state = new_state.clone();
        build_state.saved_ore -= blueprint.ore_robot_ore_cost;
        build_state.ore_robots += 1;
        max_geodes = max_geodes.max(seek(&build_state, blueprint, best_found));
        *best_found = max_geodes.max(*best_found);
    }
    if round_start_ore >= blueprint.clay_robot_ore_cost {
        let mut build_state = new_state.clone();
        build_state.saved_ore -= blueprint.clay_robot_ore_cost;
        build_state.clay_robots += 1;
        max_geodes = max_geodes.max(seek(&build_state, blueprint, best_found));
        *best_found = max_geodes.max(*best_found);
    }
    if round_start_ore >= blueprint.obsidian_robot_ore_cost
        && round_start_clay >= blueprint.obsidian_robot_clay_cost
    {
        let mut build_state = new_state.clone();
        build_state.saved_ore -= blueprint.obsidian_robot_ore_cost;
        build_state.saved_clay -= blueprint.obsidian_robot_clay_cost;
        build_state.obsidian_robots += 1;
        max_geodes = max_geodes.max(seek(&build_state, blueprint, best_found));
        *best_found = max_geodes.max(*best_found);
    }
    if round_start_ore >= blueprint.geode_robot_ore_cost
        && round_start_obsidian >= blueprint.geode_robot_obsidian_cost
    {
        let mut build_state = new_state.clone();
        build_state.saved_ore -= blueprint.geode_robot_ore_cost;
        build_state.saved_obsidian -= blueprint.geode_robot_obsidian_cost;
        build_state.geode_robots += 1;
        max_geodes = max_geodes.max(seek(&build_state, blueprint, best_found));
        *best_found = max_geodes.max(*best_found);
    }

    max_geodes
}
