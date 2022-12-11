use std::collections::VecDeque;

//#[derive (Debug)]
struct Monkey {
    items: VecDeque<u64>,
    operation: &'static dyn Fn(u64) -> u64,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

fn monkey0_op(old: u64) -> u64 {
    old * 3
}
fn monkey1_op(old: u64) -> u64 {
    old + 8
}
fn monkey2_op(old: u64) -> u64 {
    old + 2
}
fn monkey3_op(old: u64) -> u64 {
    old + 4
}
fn monkey4_op(old: u64) -> u64 {
    old * 19
}
fn monkey5_op(old: u64) -> u64 {
    old + 5
}
fn monkey6_op(old: u64) -> u64 {
    old * old
}
fn monkey7_op(old: u64) -> u64 {
    old + 1
}

fn main() {
    let monkey0 = Monkey {
        items: VecDeque::from(vec![65, 78]),
        operation: &monkey0_op,
        test: 5,
        if_true: 2,
        if_false: 3,
        inspections: 0,
    };
    let monkey1 = Monkey {
        items: VecDeque::from(vec![54, 78, 86, 79, 73, 64, 85, 88]),
        operation: &monkey1_op,
        test: 11,
        if_true: 4,
        if_false: 7,
        inspections: 0,
    };
    let monkey2 = Monkey {
        items: VecDeque::from(vec![69, 97, 77, 88, 87]),
        operation: &monkey2_op,
        test: 2,
        if_true: 5,
        if_false: 3,
        inspections: 0,
    };
    let monkey3 = Monkey {
        items: VecDeque::from(vec![99]),
        operation: &monkey3_op,
        test: 13,
        if_true: 1,
        if_false: 5,
        inspections: 0,
    };
    let monkey4 = Monkey {
        items: VecDeque::from(vec![60, 57, 52]),
        operation: &monkey4_op,
        test: 7,
        if_true: 7,
        if_false: 6,
        inspections: 0,
    };
    let monkey5 = Monkey {
        items: VecDeque::from(vec![91, 82, 85, 73, 84, 53]),
        operation: &monkey5_op,
        test: 3,
        if_true: 4,
        if_false: 1,
        inspections: 0,
    };
    let monkey6 = Monkey {
        items: VecDeque::from(vec![88, 74, 68, 56]),
        operation: &monkey6_op,
        test: 17,
        if_true: 0,
        if_false: 2,
        inspections: 0,
    };
    let monkey7 = Monkey {
        items: VecDeque::from(vec![54, 82, 72, 71, 53, 99, 67]),
        operation: &monkey7_op,
        test: 19,
        if_true: 6,
        if_false: 0,
        inspections: 0,
    };

    let mut monkeys = [
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ];
    (0..20).for_each(|_| {
        (0..8).for_each(|i| turn(&mut monkeys, i));
        (0..8).for_each(|i| println!("{:?}", monkeys[i].items));
    });
    monkeys.sort_by_key(|monkey| monkey.inspections);
    println!(
        "Part 1: {}",
        monkeys[6].inspections * monkeys[7].inspections
    );
}

fn turn(monkeys: &mut [Monkey; 8], i: usize) {
    while let Some(item) = monkeys[i].items.pop_front() {
        monkeys[i].inspections += 1;
        let (value, next_monkey) = inspection(item, &monkeys[i]);
        monkeys[next_monkey].items.push_back(value);
    }
}

fn inspection(item: u64, monkey: &Monkey) -> (u64, usize) {
    let mut result = (monkey.operation)(item);
    result = result / 3;
    if result % monkey.test == 0 {
        (result, monkey.if_true)
    } else {
        (result, monkey.if_false)
    }
}
