use lazy_static::lazy_static;
use regex::Regex;

//const VALVES: usize = 10; // Example
const VALVES: usize = 50; // Main input

#[derive(Debug, Clone, Copy)]
struct State {
    minutes_passed: u64,
    pressure_released: u64,
    current_flow: u64,
    valves_opened: [bool; VALVES],
    current_valve: usize,
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
    for valve in valves {
        valve.connected_valves = valve
            .connected_names
            .iter()
            .map(|name| names.iter().position(|n| n == name).unwrap())
            .collect();
    }
}

fn main() {
    let mut input = aoc::read_input_lines();
    input.sort();

    let mut valves: Vec<Valve> = input.iter().map(|s| s.as_str()).map(parse_line).collect();
    set_indexes(&mut valves);

    for valve in &valves {
        println!("{:?}", valve);
    }

    let mut counter: u64 = 0;
    let mut pruned: u64 = 0;

    let mut start_state = State {
        minutes_passed: 0,
        pressure_released: 0,
        current_flow: 0,
        valves_opened: [false; VALVES],
        current_valve: 0,
    };
    for (i, valve) in valves.iter().enumerate() {
        if valve.flow_rate == 0 {
            start_state.valves_opened[i] = true;
        }
    }

    println!(
        "Most pressure: {}",
        seek(start_state, &valves, 0, &mut counter, &mut pruned)
    );

    println!("It took {counter} steps and {pruned} branch prunes.");
}

const MINUTES: u64 = 30;

fn seek(state: State, valves: &[Valve], alpha: u64, counter: &mut u64, pruned: &mut u64) -> u64 {
    let mut alpha = alpha;
    *counter += 1;
    if state.minutes_passed == MINUTES {
        return state.pressure_released;
    }

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
    let all_remaining = max_gain + state.current_flow * minutes_remaining;
    let upper_bound = all_remaining + state.pressure_released;

    if *counter % 10000000 == 0 {
        println!("{counter} : {alpha} : {max_gain} : {pruned}");
    }

    // If we can't get a value that's better than has been found elsewhere, prune this branch.
    if upper_bound <= alpha {
        *pruned += 1;
        return 0;
    }

    // Explore what happens if you try to open a valve
    let opened_valve = if !state.valves_opened[state.current_valve] {
        let mut opened = state.valves_opened;
        opened[state.current_valve] = true;
        let next_state = State {
            minutes_passed: state.minutes_passed + 1,
            pressure_released: state.pressure_released + state.current_flow,
            current_flow: state.current_flow + valves[state.current_valve].flow_rate,
            valves_opened: opened,
            current_valve: state.current_valve,
        };
        let result = seek(next_state, valves, alpha, counter, pruned);
        alpha = result.max(alpha);
        result
    } else {
        0
    };

    // Explore the different tunnels to valves
    opened_valve.max(
        valves[state.current_valve]
            .connected_valves
            .iter()
            .map(|next_valve_index| {
                let next_state = State {
                    minutes_passed: state.minutes_passed + 1,
                    pressure_released: state.pressure_released + state.current_flow,
                    current_flow: state.current_flow,
                    valves_opened: state.valves_opened,
                    current_valve: *next_valve_index,
                };
                let result = seek(next_state, valves, alpha, counter, pruned);
                alpha = result.max(alpha);
                result
            })
            .max()
            .expect("Dead-end, wtf?"),
    )
}
