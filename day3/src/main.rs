use aoc::read_input_lines;
use std::collections::HashSet;

fn main() {
    let sum: u32 = read_input_lines()
        .iter()
        .map(|line| check_compartments(line))
        .sum();

    println!("Part1: {sum}");

    let sum: u32 = read_input_lines()
        .chunks(3)
        .map(|group| find_common_item(group))
        .sum();

    println!("Part2: {sum}");
}

fn find_common_item(group: &[String]) -> u32 {
    let intersection = group
        .iter()
        .map(|sack| sack.chars().collect::<HashSet<char>>())
        .reduce(|a, b| a.intersection(&b).copied().collect::<HashSet<char>>())
        .unwrap();
        let priority = intersection.iter().map(|c| priority(*c)).sum();
    println!("{intersection:?} {priority}");
    priority
}

const LOWERCASE_A: u32 = 'a' as u32;
const UPPERCASE_A: u32 = 'A' as u32;

fn check_compartments(rugsack: &str) -> u32 {
    let length = rugsack.len();
    let first = &rugsack[0..length / 2];
    let second = &rugsack[length / 2..length];

    let first_chars: HashSet<char> = first.chars().collect();
    let second_chars: HashSet<char> = second.chars().collect();
    let common = first_chars.intersection(&second_chars);

    println!("{rugsack} {length} {first} {second} {common:?}");

    let sum: u32 = common.map(|c| priority(*c)).sum();

    println!("{sum:?}");
    sum
}

fn priority(c: char) -> u32 {
    if c.is_ascii_uppercase() {
        c as u32 - UPPERCASE_A + 27
    } else {
        c as u32 - LOWERCASE_A + 1
    }
}
