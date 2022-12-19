use chrono::Local;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

const _VALVES: usize = 10; // Example
const VALVES: usize = 50; // Main input
const MINUTES: u64 = 26; // Part 2

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_passed: u64,
    pressure_released: u64,
    current_flow: u64,
    valves_opened: [bool; VALVES],
    current_valve: usize,
    elephant_valve: usize,
}

#[derive(Debug)]
struct Valve {
    name: String,
    flow_rate: u64,
    connected_valves: Vec<usize>,
    connected_names: Vec<String>,
}

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Valve (\w\w) has flow rate=(\d+); tunnels? leads? to valves? (.*)").unwrap();
}

fn parse_line(line: &str) -> Valve {
    let captures = RE.captures(line).expect("Invalid input line");

    Valve {
        name: captures[1].to_owned(),
        flow_rate: captures[2].parse().unwrap(),
        connected_valves: vec![],
        connected_names: captures[3].split(", ").map(String::from).collect(),
    }
}

fn set_indexes(valves: &mut Vec<Valve>) {
    let names: Vec<_> = valves.iter().map(|valve| &valve.name).cloned().collect();
    for valve in &mut *valves {
        valve.connected_valves = valve
            .connected_names
            .iter()
            .map(|name| names.iter().position(|n| n == name).unwrap())
            .collect();
    }

    let flow_rates = valves
        .iter()
        .map(|valve| valve.flow_rate)
        .collect::<Vec<_>>();

    for i in 0..valves.len() {
        valves[i].connected_valves.sort_by_key(|i| flow_rates[*i]);
        valves[i].connected_valves.reverse();
        println!(
            "{:?}",
            valves[i]
                .connected_valves
                .iter()
                .map(|i| flow_rates[*i])
                .collect::<Vec<_>>()
        );
    }
}

fn calc_upper_bounds(valves: &[Valve]) -> Vec<Vec<u64>> {
    let mut flow_rates = valves
        .iter()
        .map(|valve| valve.flow_rate)
        .collect::<Vec<_>>();
    flow_rates.sort();
    flow_rates.reverse();
    for rate in &flow_rates {
        println!("Flow rate: {rate}");
    }

    (0..5)
        .map(|drop| {
            (0..=(MINUTES as usize + 2))
                .map(|minutes_remaining: usize| {
                    let mut rate = 0;
                    for i in (0..(minutes_remaining.min(valves.len()))).step_by(2) {
                        rate += (minutes_remaining - i) as u64 * flow_rates[i + drop];
                        rate += (minutes_remaining - i) as u64 * flow_rates[i + drop + 1];
                    }
                    rate
                })
                .collect()
        })
        .collect()
}

fn main() {
    let mut input = aoc::read_input_lines();
    input.sort();

    let mut valves: Vec<Valve> = input.iter().map(|s| s.as_str()).map(parse_line).collect();
    set_indexes(&mut valves);

    for valve in &valves {
        println!("{:?}", valve);
    }

    let upper_bounds = calc_upper_bounds(&valves);
    for (i, bounds) in upper_bounds.iter().enumerate() {
        for (j, bound) in bounds.iter().enumerate() {
            println!("Upper bounds for {i} best valves opened and {j} minutes remaining: {bound}");
        }
    }

    let best_valves_indexes: Vec<usize> = valves
        .iter()
        .enumerate()
        .sorted_by_key(|(_, valve)| valve.flow_rate)
        .rev()
        .map(|(i, _)| i)
        .take(4)
        .collect();
    println!(
        "{best_valves_indexes:?} {:?}",
        best_valves_indexes
            .iter()
            .map(|i| &valves[*i])
            .collect_vec()
    );

    let mut counter: u64 = 0;
    let mut pruned: u64 = 0;

    let mut start_state = State {
        minutes_passed: 0,
        pressure_released: 0,
        current_flow: 0,
        valves_opened: [false; VALVES],
        current_valve: 0,
        elephant_valve: 0,
    };
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow_rate == 0 {
            start_state.valves_opened[i] = true;
        }
    }

    println!(
        "Most pressure: {}",
        seek(
            start_state,
            &valves,
            0,
            &mut counter,
            &mut pruned,
            &upper_bounds,
            &best_valves_indexes,
        )
    );

    println!("It took {counter} steps and {pruned} branch prunes.");
}

fn seek(
    state: State,
    valves: &[Valve],
    alpha: u64,
    counter: &mut u64,
    pruned: &mut u64,
    upper_bounds: &[Vec<u64>],
    best_valves_indexes: &[usize],
) -> u64 {
    let mut alpha = alpha;
    *counter += 1;
    if state.minutes_passed == MINUTES {
        return state.pressure_released;
    }

    /*

           // Get the valve with the most flow that isn't opened yet
           let max_valve = valves
               .iter()
               .enumerate()
               .filter_map(|(i, valve)| (!state.valves_opened[i]).then_some(valve.flow_rate))
               .max()
               .unwrap_or(0u64);

           let minutes_remaining = MINUTES - state.minutes_passed;
           // Carefully calculated value how much you can get if you open a valve with the max flow valve every other minute.
           let max_gain = (minutes_remaining * minutes_remaining / 4) * max_valve;
           let max_gain_with_elephant = max_gain * 2; // There's probably a lower upper bound.
           let all_remaining = max_gain_with_elephant + state.current_flow * minutes_remaining;
           let upper_bound = all_remaining + state.pressure_released;
    */

    let minutes_remaining = MINUTES - state.minutes_passed;
    /*

        let upper_bound = (match best_valves_indexes
            .iter()
            .map(|i| state.valves_opened[*i])
            .collect_vec()[0..4]
        {
            [true, true, true, true] => &upper_bounds[4],
            [true, true, true, _] => &upper_bounds[3],
            [true, true, _, _] => &upper_bounds[2],
            [true, _, _, _] => &upper_bounds[1],
            _ => &upper_bounds[0],
        }[minutes_remaining as usize + 1]
            + state.pressure_released
            + 5).min(2400);
    */

    let upper_bound =
        (upper_bounds[0][minutes_remaining as usize + 1] + state.pressure_released + 5);

    if state.minutes_passed <= 4 {
        println!(
            "{}: {}, {counter} : {alpha} : {upper_bound} : {pruned} : {}",
            Local::now().format("%H:%M:%S"),
            state.minutes_passed,
            *pruned as f64 / *counter as f64,
        );
    }

    // If we can't get a value that's better than has been found elsewhere, prune this branch.
    if upper_bound <= alpha {
        *pruned += 1;
        return 0;
    }

    let left: Vec<usize>;
    let right: Vec<usize>;

    let (human_moves, elephant_moves) = if state.current_valve == state.elephant_valve {
        let next_valves = &valves[state.current_valve].connected_valves;

        let (a, b): (Vec<_>, Vec<_>) = next_valves
            .iter()
            .copied()
            .tuple_combinations::<(usize, usize)>()
            .unzip();
        left = a.iter().copied().unique().collect::<Vec<_>>();
        right = b.iter().copied().unique().collect::<Vec<_>>();

        (&left, &right)
    } else {
        (
            &valves[state.current_valve].connected_valves,
            &valves[state.elephant_valve].connected_valves,
        )
    };

    // Human tries to open a valve
    if !state.valves_opened[state.current_valve] {
        let mut opened = state.valves_opened;
        opened[state.current_valve] = true;
        let current_flow = state.current_flow + valves[state.current_valve].flow_rate;

        // Elephant tries to open a valve at the same time
        if !opened[state.elephant_valve] {
            opened[state.elephant_valve] = true;
            let next_state = State {
                minutes_passed: state.minutes_passed + 1,
                pressure_released: state.pressure_released + state.current_flow,
                current_flow: current_flow + valves[state.elephant_valve].flow_rate,
                valves_opened: opened,
                current_valve: state.current_valve,
                elephant_valve: state.elephant_valve,
            };
            let result = seek(
                next_state,
                valves,
                alpha,
                counter,
                pruned,
                upper_bounds,
                best_valves_indexes,
            );
            alpha = result.max(alpha);
        }
        // Elephant moves at the same time
        for elephant_valve in elephant_moves {
            let next_state = State {
                minutes_passed: state.minutes_passed + 1,
                pressure_released: state.pressure_released + state.current_flow,
                current_flow: current_flow,
                valves_opened: opened,
                current_valve: state.current_valve,
                elephant_valve: *elephant_valve,
            };
            let result = seek(
                next_state,
                valves,
                alpha,
                counter,
                pruned,
                upper_bounds,
                best_valves_indexes,
            );
            alpha = result.max(alpha);
        }
    }

    // Human moves
    for human_valve in human_moves {
        // Elephant tries to open a valve at the same time
        if !state.valves_opened[state.elephant_valve] {
            let mut opened = state.valves_opened;
            opened[state.elephant_valve] = true;
            let next_state = State {
                minutes_passed: state.minutes_passed + 1,
                pressure_released: state.pressure_released + state.current_flow,
                current_flow: state.current_flow + valves[state.elephant_valve].flow_rate,
                valves_opened: opened,
                current_valve: *human_valve,
                elephant_valve: state.elephant_valve,
            };
            let result = seek(
                next_state,
                valves,
                alpha,
                counter,
                pruned,
                upper_bounds,
                best_valves_indexes,
            );
            alpha = result.max(alpha);
        }
        // Elephant moves at the same time
        for elephant_valve in elephant_moves {
            let next_state = State {
                minutes_passed: state.minutes_passed + 1,
                pressure_released: state.pressure_released + state.current_flow,
                current_flow: state.current_flow,
                valves_opened: state.valves_opened,
                current_valve: *human_valve,
                elephant_valve: *elephant_valve,
            };
            let result = seek(
                next_state,
                valves,
                alpha,
                counter,
                pruned,
                upper_bounds,
                best_valves_indexes,
            );
            alpha = result.max(alpha);
        }
    }
    alpha
}
