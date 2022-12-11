use core::fmt;
use std::collections::VecDeque;

struct Monkey {
    items: VecDeque<u64>,
    operation: Operation,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

impl Monkey {
    fn parse(lines: &[String], i: usize) -> Monkey {
        let items: VecDeque<u64> = lines[7 * i + 1][18..]
            .split(", ")
            .filter_map(|part| part.parse().ok())
            .collect();

        let operation = Operation::parse(&lines[7 * i + 2][23..]);
        let test: u64 = lines[7 * i + 3][21..].parse().unwrap();
        let if_true: usize = lines[7 * i + 4][29..].parse().unwrap();
        let if_false: usize = lines[7 * i + 5][30..].parse().unwrap();

        Monkey {
            items,
            operation,
            test,
            if_true,
            if_false,
            inspections: 0,
        }
    }
}

impl fmt::Debug for Monkey {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use fmt::Write;

        let faces = ['🙈', '🙉', '🙊'];
        let i = fastrand::usize(..faces.len());
        f.write_char(faces[i])
    }
}

enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn parse(ops: &str) -> Operation {
        match ops.split_once(' ').unwrap() {
            ("*", "old") => Operation::Square,
            ("*", a) => Operation::Multiply(a.parse().unwrap()),
            ("+", a) => Operation::Add(a.parse().unwrap()),
            (a, b) => panic!("Odd operation {a} {b}"),
        }
    }

    fn calc(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Multiply(x) => old * x,
            Operation::Square => old * old,
        }
    }
}

const N: usize = 8;
const ROUNDS: usize = 10000;

fn main() {
    let input = aoc::read_input_lines();

    let mut troop: [Monkey; N] = (0..N)
        .map(|i| Monkey::parse(&input, i))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let cap = troop.iter().map(|monkey| monkey.test).product();

    (0..ROUNDS).for_each(|_| {
        (0..N).for_each(|i| turn(&mut troop, i, cap));
    });
    troop.sort_by_key(|monkey| monkey.inspections);
    println!("{troop:?}");
    println!(
        "Result: {}",
        troop[N - 2].inspections * troop[N - 1].inspections
    );
}

fn turn(troop: &mut [Monkey; N], i: usize, cap: u64) {
    while let Some(item) = troop[i].items.pop_front() {
        troop[i].inspections += 1;
        let (value, next_monkey) = inspection(item, &troop[i], cap);
        troop[next_monkey].items.push_back(value);
    }
}

fn inspection(item: u64, monkey: &Monkey, cap: u64) -> (u64, usize) {
    let result = monkey.operation.calc(item) % cap;
    if result % monkey.test == 0 {
        (result, monkey.if_true)
    } else {
        (result, monkey.if_false)
    }
}

fn _inspection_part1(item: u64, monkey: &Monkey) -> (u64, usize) {
    let mut result = monkey.operation.calc(item);
    result /= 3;
    if result % monkey.test == 0 {
        (result, monkey.if_true)
    } else {
        (result, monkey.if_false)
    }
}
