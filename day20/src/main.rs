use log::info;
use std::fmt::Display;

#[derive(Clone, Debug)]
struct Item {
    original_index: usize,
    value: i64,
}

impl Item {
    fn new(original_index: usize, value: i64) -> Item {
        Item {
            original_index,
            value,
        }
    }
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
        .map(|(i, value)| Item::new(i, value.parse::<i64>().unwrap()))
        .collect();

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
    let thousand = &buffer[(pos_zero + 1000).rem_euclid(buffer.len())];
    let two_thousand = &buffer[(pos_zero + 2000).rem_euclid(buffer.len())];
    let three_thousand = &buffer[(pos_zero + 3000).rem_euclid(buffer.len())];

    info!(
        "{} + {} + {} = {}",
        thousand,
        two_thousand,
        three_thousand,
        thousand.value + two_thousand.value + three_thousand.value
    );
}

fn mix(buffer: &mut Vec<Item>) {
    for i in 0..buffer.len() {
        let i = buffer.iter().position(|a| a.original_index == i).unwrap();
        shift(buffer, i);
    }
}

fn shift(buffer: &mut Vec<Item>, i: usize) {
    let n = buffer.len();
    let moves = buffer[i].value.rem_euclid(n as i64 - 1) as usize;
    for j in 0..moves {
        buffer.swap((i + j).rem_euclid(n), (i + j + 1).rem_euclid(n))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[rstest]
    #[case(vec![1, 2, -3, 3, -2, 0, 4],0,vec![2, 1, -3, 3, -2, 0, 4])]
    #[case(vec![2, 1, -3, 3, -2, 0, 4],0,vec![1, -3, 2, 3, -2, 0, 4])]
    #[case(vec![1, -3, 2, 3, -2, 0, 4],1,vec![1, 2, 3, -2, -3, 0, 4])]
    #[case(vec![1, 2, 3, -2, -3, 0, 4],2,vec![1, 2, -2, -3, 0, 3, 4])]
    #[case(vec![1, 2, -2, -3, 0, 3, 4],2,vec![1, 2, -3, 0, 3, 4, -2])]
    #[case(vec![1, 2, -3, 0, 3, 4, -2],3,vec![1, 2, -3, 0, 3, 4, -2])]
    #[case(vec![1, 2, -3, 0, 3, 4, -2],5,vec![2, -3, 4, 0, 3, -2, 1])]
    fn case1(#[case] input: Vec<i64>, #[case] index: usize, #[case] output: Vec<i64>) {
        let mut buffer = input.iter().map(|i| Item::new(0, *i)).collect();
        shift(&mut buffer, index);
        assert_eq!(buffer.iter().map(|i| i.value).collect::<Vec<_>>(), output)
    }
}
