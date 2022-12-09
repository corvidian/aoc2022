use std::cmp::Ordering;
use std::fmt::Debug;
use std::{collections::HashSet, hash::Hash};

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Position(i32, i32);

impl Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", &self.0, &self.1)
    }
}

fn main() {
    let mut head = Position(0, 0);
    let mut tail = [Position(0, 0); 9];
    let mut visited: HashSet<Position> = HashSet::new();
    visited.insert(tail[8].clone());
    aoc::read_input_lines()
        .iter()
        .for_each(|line| instruction(line, &mut head, &mut tail, &mut visited));

    println!("{visited:?}");

    println!("Part 1: {}", visited.len());
}

fn instruction(
    instruction: &str,
    head: &mut Position,
    tail: &mut [Position; 9],
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
    (0..amount).for_each(|_| movement(&delta, head, tail, visited));
}

fn movement(
    delta: &Position,
    head: &mut Position,
    tail: &mut [Position; 9],
    visited: &mut HashSet<Position>,
) {
    head.0 = head.0 + delta.0;
    head.1 = head.1 + delta.1;

    move_tail(head, &mut tail[0]);
    for i in 1..9 {
        move_tail(&tail[i - 1].clone(), &mut tail[i]);
    }

    println!("head: {head:?} tail: {tail:?}");

    visited.insert(tail[8].clone());
}

fn move_tail(head: &Position, tail: &mut Position) {
    if (head.0 - tail.0).abs() > 1 || (head.1 - tail.1).abs() > 1 {
        match head.0.cmp(&tail.0) {
            Ordering::Less => tail.0 -= 1,
            Ordering::Greater => tail.0 += 1,
            Ordering::Equal => tail.0 += 0,
        }

        match head.1.cmp(&tail.1) {
            Ordering::Less => tail.1 -= 1,
            Ordering::Greater => tail.1 += 1,
            Ordering::Equal => tail.1 += 0,
        }
    }
}
