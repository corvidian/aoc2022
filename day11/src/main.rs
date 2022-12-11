use num_bigint::BigUint;
use num_traits::Zero;
use std::collections::VecDeque;

//#[derive (Debug)]
struct Monkey {
    items: VecDeque<BigUint>,
    operation: &'static dyn Fn(BigUint) -> BigUint,
    test: u64,
    if_true: usize,
    if_false: usize,
    inspections: u64,
}

fn monkey0_op(old: BigUint) -> BigUint {
    old * 3u8
}
fn monkey1_op(old: BigUint) -> BigUint {
    old + 8u8
}
fn monkey2_op(old: BigUint) -> BigUint {
    old + 2u8
}
fn monkey3_op(old: BigUint) -> BigUint {
    old + 4u8
}
fn monkey4_op(old: BigUint) -> BigUint {
    old * 19u8
}
fn monkey5_op(old: BigUint) -> BigUint {
    old + 5u8
}
fn monkey6_op(old: BigUint) -> BigUint {
    old.pow(2)
}
fn monkey7_op(old: BigUint) -> BigUint {
    old + 1u8
}

const N: usize = 8;

fn main() {
    let monkey0 = Monkey {
        items: VecDeque::from(vec![65u32.into(), 78u32.into()]),
        operation: &monkey0_op,
        test: 5,
        if_true: 2,
        if_false: 3,
        inspections: 0,
    };
    let monkey1 = Monkey {
        items: VecDeque::from(vec![
            54u32.into(),
            78u32.into(),
            86u32.into(),
            79u32.into(),
            73u32.into(),
            64u32.into(),
            85u32.into(),
            88u32.into(),
        ]),
        operation: &monkey1_op,
        test: 11,
        if_true: 4,
        if_false: 7,
        inspections: 0,
    };
    let monkey2 = Monkey {
        items: VecDeque::from(vec![
            69u32.into(),
            97u32.into(),
            77u32.into(),
            88u32.into(),
            87u32.into(),
        ]),
        operation: &monkey2_op,
        test: 2,
        if_true: 5,
        if_false: 3,
        inspections: 0,
    };
    let monkey3 = Monkey {
        items: VecDeque::from(vec![99u32.into()]),
        operation: &monkey3_op,
        test: 13,
        if_true: 1,
        if_false: 5,
        inspections: 0,
    };
    let monkey4 = Monkey {
        items: VecDeque::from(vec![60u32.into(), 57u32.into(), 52u32.into()]),
        operation: &monkey4_op,
        test: 7,
        if_true: 7,
        if_false: 6,
        inspections: 0,
    };
    let monkey5 = Monkey {
        items: VecDeque::from(vec![
            91u32.into(),
            82u32.into(),
            85u32.into(),
            73u32.into(),
            84u32.into(),
            53u32.into(),
        ]),
        operation: &monkey5_op,
        test: 3,
        if_true: 4,
        if_false: 1,
        inspections: 0,
    };
    let monkey6 = Monkey {
        items: VecDeque::from(vec![88u32.into(), 74u32.into(), 68u32.into(), 56u32.into()]),
        operation: &monkey6_op,
        test: 17,
        if_true: 0,
        if_false: 2,
        inspections: 0,
    };
    let monkey7 = Monkey {
        items: VecDeque::from(vec![
            54u32.into(),
            82u32.into(),
            72u32.into(),
            71u32.into(),
            53u32.into(),
            99u32.into(),
            67u32.into(),
        ]),
        operation: &monkey7_op,
        test: 19,
        if_true: 6,
        if_false: 0,
        inspections: 0,
    };

    let mut monkeys = [
        monkey0, monkey1, monkey2, monkey3, monkey4, monkey5, monkey6, monkey7,
    ];
    (0..10000).for_each(|round| {
        (0..8).for_each(|i| turn(&mut monkeys, i));
        println!("{round}");
//        (0..8).for_each(|i| println!("{:?}", monkeys[i].items));
    });
    monkeys.sort_by_key(|monkey| monkey.inspections);
    println!(
        "Part 1: {}",
        monkeys[N - 2].inspections * monkeys[N - 1].inspections
    );
}

fn turn(monkeys: &mut [Monkey; N], i: usize) {
    while let Some(item) = monkeys[i].items.pop_front() {
        monkeys[i].inspections += 1;
        let (value, next_monkey) = inspection(item, &monkeys[i]);
        monkeys[next_monkey].items.push_back(value);
    }
}

fn inspection(item: BigUint, monkey: &Monkey) -> (BigUint, usize) {
    let result = (monkey.operation)(item);
    //result = result / 3;
    if result.clone() % monkey.test == Zero::zero() {
        (result, monkey.if_true)
    } else {
        (result, monkey.if_false)
    }
}
