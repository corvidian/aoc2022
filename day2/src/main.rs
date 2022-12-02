use std::collections::HashMap;
use aoc::read_input_lines;

fn main() {
    let game_points = HashMap::from([
        ("A X".to_string(), (4, 3)),
        ("B X".to_string(), (1, 1)),
        ("C X".to_string(), (7, 2)),
        ("A Y".to_string(), (8, 4)),
        ("B Y".to_string(), (5, 5)),
        ("C Y".to_string(), (2, 6)),
        ("A Z".to_string(), (3, 8)),
        ("B Z".to_string(), (9, 9)),
        ("C Z".to_string(), (6, 7)),
    ]);

    let sums: (i32, i32) = read_input_lines()
        .iter()
        .map(|line| game_points[line])
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
        .expect("Empty input file");
    println!("Part 1 Sum: {}", sums.0);
    println!("Part 2 Sum: {}", sums.1);
}
