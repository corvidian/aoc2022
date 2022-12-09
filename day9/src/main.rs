use std::cmp::Ordering;
use std::fmt::Debug;
use std::{collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone)]
struct Position(i32, i32);

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", &self.0, &self.1)
    }
}

fn main() {
    let input = aoc::read_input_lines();
    let mut rope = [Position; 2].map(|_| Position(0,0));
    let mut visited: HashSet<Position> = HashSet::new();
    input
        .iter()
        .for_each(|line| instruction(line, &mut rope, &mut visited));

    println!("Part 1: {}", visited.len());

    let mut rope = [Position; 10].map(|_| Position(0,0));
    visited.clear();
    input
        .iter()
        .for_each(|line| instruction(line, &mut rope, &mut visited));

    println!("Part 2: {}", visited.len());
}

fn instruction<const N: usize>(
    instruction: &str,
    rope: &mut [Position; N],
    visited: &mut HashSet<Position>,
) {
    let direction = instruction.chars().next().unwrap();
    let delta = match direction {
        'R' => Position(1, 0),
        'L' => Position(-1, 0),
        'U' => Position(0, 1),
        'D' => Position(0, -1),
        _ => panic!("Invalid instruction"),
    };
    let amount: usize = instruction
        .split_once(' ')
        .unwrap()
        .1
        .parse()
        .expect("Invalid move amount");
    (0..amount).for_each(|_| movement(&delta, rope, visited));
}

fn movement<const N: usize>(
    delta: &Position,
    rope: &mut [Position; N],
    visited: &mut HashSet<Position>,
) {
    rope[0].0 += delta.0;
    rope[0].1 += delta.1;

    for i in 1..N {
        move_rope(&rope[i - 1].clone(), &mut rope[i]);
    }

    visited.insert(rope[N - 1].clone());
}

fn move_rope(head: &Position, rope: &mut Position) {
    if (head.0 - rope.0).abs() > 1 || (head.1 - rope.1).abs() > 1 {
        match head.0.cmp(&rope.0) {
            Ordering::Less => rope.0 -= 1,
            Ordering::Greater => rope.0 += 1,
            Ordering::Equal => rope.0 += 0,
        }

        match head.1.cmp(&rope.1) {
            Ordering::Less => rope.1 -= 1,
            Ordering::Greater => rope.1 += 1,
            Ordering::Equal => rope.1 += 0,
        }
    }
}
