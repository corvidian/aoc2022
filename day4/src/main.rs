use aoc::read_input_lines;
use std::collections::HashSet;

fn main() {
    let matches: Vec<(bool, bool)> = read_input_lines()
        .iter()
        .map(|line| check_subset(line))
        .collect();

    let part1 = matches.iter().filter(|(b, _)| *b).count();
    println!("Part 1: {part1}");

    let part2 = matches.iter().filter(|(_, b)| *b).count();
    println!("Part 2: {part2}");
}

fn check_subset(line: &str) -> (bool, bool) {
    let sets = line.split(',').map(to_set).collect::<Vec<HashSet<u32>>>();
    (
        sets[0].is_subset(&sets[1]) || sets[0].is_superset(&sets[1]),
        !sets[0].is_disjoint(&sets[1]),
    )
}

fn to_set(s: &str) -> HashSet<u32> {
    let ends = s
        .split('-')
        .map(|c| c.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    (ends[0]..=ends[1]).collect()
}
