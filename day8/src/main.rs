use itertools::Itertools;
use take_until::TakeUntilExt;

fn main() {
    let lines = aoc::read_input_lines();
    let width = lines[0].len();
    let height = lines.len();

    let height_map: Vec<Vec<u8>> = lines
        .iter()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as u8)
                .collect::<Vec<u8>>()
        })
        .collect::<Vec<_>>();

    visualize_height(&height_map);

    let part1 = part1(height, width, &height_map);
    let part2 = part2(height, width, &height_map);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn check_visibility<'a, I, F>(selected: &u8, indexes: I, value: F) -> usize
where
    I: Iterator<Item = usize>,
    F: Fn(usize) -> &'a u8,
{
    indexes.take_until(|&x| value(x) >= selected).count()
}

fn part2(height: usize, width: usize, height_map: &[Vec<u8>]) -> usize {
    (1..height - 1)
        .cartesian_product(1..width - 1)
        .map(|(j, i)| {
            let selected = height_map[j][i];

            let counter_right = check_visibility(&selected, (i + 1)..width, |x| &height_map[j][x]);
            let counter_left = check_visibility(&selected, (0..i).rev(), |x| &height_map[j][x]);
            let counter_down = check_visibility(&selected, (j + 1)..height, |y| &height_map[y][i]);
            let counter_up = check_visibility(&selected, (0..j).rev(), |y| &height_map[y][i]);

            counter_down * counter_right * counter_up * counter_left
        })
        .max()
        .unwrap()
}

fn part1(height: usize, width: usize, height_map: &[Vec<u8>]) -> u32 {
    let mut visible_map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 0..width {
        visible_map[0][i] = true;
        visible_map[height - 1][i] = true;
    }

    for line in visible_map.iter_mut() {
        line[0] = true;
        line[width - 1] = true;
    }

    let mut highest;

    for y in 1..height - 1 {
        highest = height_map[y][0];
        for x in 1..width - 1 {
            if height_map[y][x] > highest {
                highest = height_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    for y in 1..height - 1 {
        highest = height_map[y][width - 1];
        for x in 2..width {
            if height_map[y][width - x] > highest {
                highest = height_map[y][width - x];
                visible_map[y][width - x] = true;
            }
        }
    }

    for x in 1..width - 1 {
        highest = height_map[0][x];
        for y in 1..height - 1 {
            if height_map[y][x] > highest {
                highest = height_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    for x in 1..width - 1 {
        highest = height_map[height - 1][x];
        for y in 2..height {
            if height_map[height - y][x] > highest {
                highest = height_map[height - y][x];
                visible_map[height - y][x] = true;
            }
        }
    }

    visualize_visibility(&visible_map);
    println!();

    visible_map
        .iter()
        .map(|line| line.iter().map(|c| u32::from(*c)).sum::<u32>())
        .sum()
}

fn visualize_visibility(map: &[Vec<bool>]) {
    map.iter().for_each(|line| draw_line_visibility(line))
}

fn draw_line_visibility(line: &[bool]) {
    line.iter().for_each(|c| {
        print!("{}", u32::from(*c));
    });
    println!()
}

fn visualize_height(map: &[Vec<u8>]) {
    map.iter().for_each(|line| draw_line_height(line));
    println!()
}

fn draw_line_height(line: &[u8]) {
    line.iter().for_each(|c| print!("{c}"));
    println!()
}
