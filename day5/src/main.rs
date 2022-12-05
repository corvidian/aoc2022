use itertools::Itertools;

fn main() {
    let input = aoc::read_input_string();
    let (diagram, commands) = input.split_once("\n\n").unwrap();

    let commands = commands.lines().map(parse_command).collect::<Vec<_>>();

    let mut stacks = find_stacks(diagram);
    let mut stacks2 = stacks.clone();

    for command in &commands {
        do_command(&mut stacks, command, false)
    }
    println!("Part 1: {}", get_answer(&stacks));

    for command in &commands {
        do_command(&mut stacks2, command, true)
    }
    println!("Part 2: {}", get_answer(&stacks2));
}

fn get_answer(stacks: &[String]) -> String {
    stacks
        .iter()
        .map(|s| s.chars().last().unwrap())
        .collect::<String>()
}

fn do_command(stacks: &mut [String], command: &(usize, usize, usize), part2: bool) {
    let (n, from, to) = command.to_owned();
    let mut items: Vec<_> = (0..n).map(|_| stacks[from - 1].pop().unwrap()).collect();
    if part2 {
        items.reverse();
    }
    stacks[to - 1].push_str(&String::from_iter(items))
}

fn parse_command(line: &str) -> (usize, usize, usize) {
    line.split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple()
        .unwrap()
}

fn find_stacks(diagram: &str) -> Vec<String> {
    let stack_count = diagram.lines().last().unwrap().split("  ").count();

    let mut stacks = (0..stack_count).map(|_| "".to_string()).collect::<Vec<_>>();

    for line in diagram.lines().rev().skip(1) {
        for (i, c) in line.chars().enumerate() {
            if c.is_alphabetic() {
                stacks[(i - 1) / 4].push(c);
            }
        }
    }

    stacks
}
