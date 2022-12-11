use std::collections::VecDeque;

#[derive(Debug)]
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
        let a: [&str; 2] = lines[7 * i + 2]
            .trim()
            .split(' ')
            .skip(4)
            .take(2)
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();
        let operation = match a {
            ["*", "old"] => Operation::Square,
            ["*", a] => Operation::Multiply(a.parse().unwrap()),
            ["+", a] => Operation::Add(a.parse().unwrap()),
            [a, b] => panic!("Odd operation {a} {b}"),
        };

        let items = lines[7 * i + 1]
            .split(&[',', ' '])
            .into_iter()
            .filter_map(|part| part.parse::<u64>().ok())
            .collect::<VecDeque<_>>();

        let test = lines[7 * i + 3]
            .split(' ')
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap();

        let if_true = lines[7 * i + 4]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

        let if_false = lines[7 * i + 5]
            .split(' ')
            .last()
            .unwrap()
            .parse::<usize>()
            .unwrap();

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

#[derive(Debug)]
enum Operation {
    Add(u64),
    Multiply(u64),
    Square,
}

impl Operation {
    fn calc(&self, old: u64) -> u64 {
        match self {
            Operation::Add(x) => old + x,
            Operation::Multiply(x) => old * x,
            Operation::Square => old * old,
        }
    }
}

const N: usize = 8;

fn main() {
    let input = aoc::read_input_lines();

    let mut troop: [Monkey; N] = (0..8)
        .map(|i| Monkey::parse(&input, i))
        .collect::<Vec<_>>()
        .try_into()
        .unwrap();

    let cap = troop
        .iter()
        .map(|monkey| monkey.test)
        .product();

    (0..10000).for_each(|_| {
        (0..N).for_each(|i| turn(&mut troop, i, cap));
    });
    troop.sort_by_key(|monkey| monkey.inspections);
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
