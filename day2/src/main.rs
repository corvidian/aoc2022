use aoc::input_lines;
use log::info;
use std::collections::HashMap;

fn main() {
    aoc::init_logging();

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

    let sums: (i32, i32) = input_lines()
        .map(|line| game_points[&line])
        .reduce(|acc, x| (acc.0 + x.0, acc.1 + x.1))
        .expect("Empty input file");
    info!("Part 1 Sum: {}", sums.0);
    info!("Part 2 Sum: {}", sums.1);
}
