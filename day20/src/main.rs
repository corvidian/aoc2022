use itertools::Itertools;
use log::{debug, info};
use std::{collections::VecDeque, fmt::Display};

#[derive(Clone, Copy, Debug)]
struct Item {
    original_index: usize,
    value: i64,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{} -> {}", self.original_index, self.value))
    }
}

const DECRYPT_KEY: i64 = 811589153;

fn main() {
    aoc::init_logging();
    let mut buffer: VecDeque<_> = aoc::input_lines()
        .enumerate()
        .map(|(i, value)| Item {
            original_index: i,
            value: value.parse::<i64>().unwrap() * DECRYPT_KEY,
        })
        .collect();

    debug!("{}", buffer.iter().join(", "));

    let file_length = buffer.len() as i64;

    for _ in 0..10 {
        for i in 0..buffer.len() {
            let i = buffer
                .iter()
                .position(|a| a.original_index == i)
                .unwrap();
            let a = buffer.remove(i).unwrap();

            let new_i = (i as i64 + a.value).rem_euclid(file_length - 1) as usize;

            debug!(
                "Index for {a} is {new_i} (= {i} + {} % {})",
                a.value,
                file_length - 1
            );

            buffer.insert(new_i, a);
            debug!("{}", buffer.iter().map(|a| a.value).join(", "));
        }
    }

    let pos_zero = buffer.iter().position(|a| a.value == 0).unwrap();
    let thousand = buffer[(pos_zero + 1000).rem_euclid(file_length as usize)];
    let two_thousand = buffer[(pos_zero + 2000).rem_euclid(file_length as usize)];
    let three_thousand = buffer[(pos_zero + 3000).rem_euclid(file_length as usize)];

    info!(
        "{} + {} + {} = {}",
        thousand,
        two_thousand,
        three_thousand,
        thousand.value + two_thousand.value + three_thousand.value
    );
}
