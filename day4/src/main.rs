use aoc::read_input_lines;
use itertools::Itertools;
use std::collections::HashSet;

fn main() {
    let sets = read_input_lines()
        .iter()
        .map(|line| line_to_sets(line))
        .collect::<Vec<(HashSet<u32>, HashSet<u32>)>>();

    let part1 = sets
        .iter()
        .filter(|pair| pair.0.is_subset(&pair.1) || pair.0.is_superset(&pair.1))
        .count();
    println!("Part 1: {part1}");

    let part2 = sets
        .iter()
        .filter(|pair| !pair.0.is_disjoint(&pair.1))
        .count();
    println!("Part 2: {part2}");
}

fn line_to_sets(line: &str) -> (HashSet<u32>, HashSet<u32>) {
    line.split(',')
        .map(to_set)
        .into_iter()
        .collect_tuple()
        .expect("More than two elves in a group")
}

fn to_set(s: &str) -> HashSet<u32> {
    let (start, end) = s
        .split('-')
        .map(|c| c.parse::<u32>().unwrap())
        .collect_tuple()
        .expect("More than two numbers in a range");

    (start..=end).collect()
}
