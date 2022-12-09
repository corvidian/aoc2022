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

const fn new_pos() -> Position {
    Position(0, 0)
}

fn main() {
    const NEW_POS: Position = new_pos();

    let input = aoc::read_input_lines();
    let mut head = new_pos();
    let mut tail = [NEW_POS; 1];
    let mut visited: HashSet<Position> = HashSet::new();
    input
        .iter()
        .for_each(|line| instruction(line, &mut head, &mut tail, &mut visited));

    println!("Part 1: {}", visited.len());

    head = new_pos();
    let mut tail = [NEW_POS; 9];
    visited.clear();
    input
        .iter()
        .for_each(|line| instruction(line, &mut head, &mut tail, &mut visited));

    println!("Part 2: {}", visited.len());
}

fn instruction<const N: usize>(
    instruction: &str,
    head: &mut Position,
    tail: &mut [Position; N],
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

fn movement<const N: usize>(
    delta: &Position,
    head: &mut Position,
    tail: &mut [Position; N],
    visited: &mut HashSet<Position>,
) {
    head.0 += delta.0;
    head.1 += delta.1;

    move_tail(head, &mut tail[0]);
    for i in 1..N {
        move_tail(&tail[i - 1].clone(), &mut tail[i]);
    }

    visited.insert(tail[N - 1].clone());
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
