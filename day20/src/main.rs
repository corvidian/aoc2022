use itertools::Itertools;
use log::{debug, info};
use std::fmt::Display;

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
    let mut file: Vec<_> = aoc::input_lines()
        .enumerate()
        .map(|(i, value)| Item {
            original_index: i,
            value: value.parse::<i64>().unwrap(),
        })
        .collect();

    debug!("{}", file.iter().join(", "));

    let mut part1_buffer = file.clone();
    mix(&mut part1_buffer);
    result(&part1_buffer);

    for mut item in &mut file {
        item.value = item.value * DECRYPT_KEY
    }

    for _ in 0..10 {
        mix(&mut file)
    }
    result(&file);
}

fn result(buffer: &Vec<Item>) {
    let pos_zero = buffer.iter().position(|a| a.value == 0).unwrap();
    let thousand = buffer[(pos_zero + 1000).rem_euclid(buffer.len())];
    let two_thousand = buffer[(pos_zero + 2000).rem_euclid(buffer.len())];
    let three_thousand = buffer[(pos_zero + 3000).rem_euclid(buffer.len())];

    info!(
        "{} + {} + {} = {}",
        thousand,
        two_thousand,
        three_thousand,
        thousand.value + two_thousand.value + three_thousand.value
    );
}

fn mix(buffer: &mut Vec<Item>) {
    let modulo = buffer.len() as i64 - 1;
    for i in 0..buffer.len() {
        let i = buffer.iter().position(|a| a.original_index == i).unwrap();
        let a = buffer.remove(i);

        let new_i = (i as i64 + a.value).rem_euclid(modulo) as usize;

        debug!(
            "Index for {a} is {new_i} (= {i} + {} % {})",
            a.value, modulo
        );

        buffer.insert(new_i, a);
        debug!("{}", buffer.iter().map(|a| a.value).join(", "));
    }
}
