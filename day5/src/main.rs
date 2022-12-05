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

fn get_answer(stacks: &[Vec<char>]) -> String {
    stacks.iter().map(|s| s.last().unwrap()).collect::<String>()
}

fn do_command(stacks: &mut [Vec<char>], command: &(usize, usize, usize), part2: bool) {
    let (amount, from, to) = command;
    let mut items = (0..*amount)
        .map(|_| stacks[*from - 1].pop().unwrap())
        .collect::<Vec<_>>();
    if part2 {
        items.reverse();
    }
    for item in items {
        stacks[*to - 1].push(item)
    }
}

fn parse_command(line: &str) -> (usize, usize, usize) {
    line.split(' ')
        .filter_map(|s| s.parse::<usize>().ok())
        .collect_tuple()
        .unwrap()
}

fn find_stacks(diagram: &str) -> Vec<Vec<char>> {
    let stack_count = diagram.lines().last().unwrap().split("  ").count();

    let mut stacks = (0..stack_count).map(|_| vec![]).collect::<Vec<_>>();

    for line in diagram.lines().rev().skip(1) {
        for (x, stack) in stacks.iter_mut().enumerate() {
            let char = line.chars().nth(x * 4 + 1).unwrap();
            if char.is_alphabetic() {
                stack.push(char)
            }
        }
    }

    stacks
}
