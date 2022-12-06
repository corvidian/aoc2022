use std::collections::HashSet;

fn main() {
    println!("Part 1: {}", find_marker(4));
    println!("Part 2: {}", find_marker(14));
}

fn find_marker(n: usize) -> usize {
    aoc::read_input_string()
        .chars()
        .collect::<Vec<_>>()
        .windows(n)
        .enumerate()
        .find(|(_, window)| HashSet::<char>::from_iter(window.iter().copied()).len() == n)
        .unwrap()
        .0
        + n
}
