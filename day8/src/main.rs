use std::ops::{Index, IndexMut};

use itertools::Itertools;
use take_until::TakeUntilExt;

trait Visualize {
    fn cell(&self, y: usize, x: usize) -> u8;
    fn height(&self) -> usize;
    fn width(&self) -> usize;

    fn visualize(&self) {
        for y in 0..self.height() {
            for x in 0..self.width() {
                print!("{}", self.cell(y, x));
            }
            println!();
        }
        println!();
    }
}

struct HeightMap {
    height: usize,
    width: usize,
    map: Vec<u8>,
}

impl HeightMap {
    fn new(lines: Vec<String>) -> HeightMap {
        let height = lines.len();
        let width = lines[0].len();
        let mut this = HeightMap {
            height,
            width,
            map: vec![0; height * width],
        };

        for (y, line) in lines.iter().enumerate().take(height) {
            for (x, char) in line.chars().enumerate() {
                this.map[y * width + x] = char.to_digit(10).unwrap() as u8;
            }
        }

        this
    }
}

impl Index<usize> for HeightMap {
    type Output = [u8];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        &self.map[start..start + self.width]
    }
}

impl Visualize for HeightMap {
    fn cell(&self, y: usize, x: usize) -> u8 {
        self[y][x]
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

struct VisibilityMap {
    height: usize,
    width: usize,
    map: Vec<bool>,
}

impl VisibilityMap {
    fn new(height: usize, width: usize) -> VisibilityMap {
        let mut this = VisibilityMap {
            height,
            width,
            map: vec![false; height * width],
        };

        for x in 0..width {
            this.set(0, x);
            this.set(height - 1, x);
        }

        for y in 0..height {
            this.set(y, 0);
            this.set(y, width - 1);
        }

        this
    }

    fn set(&mut self, y: usize, x: usize) {
        self[y][x] = true;
    }

    fn count(&self) -> u32 {
        self.map.iter().map(|b| u32::from(*b)).sum()
    }
}

impl Visualize for VisibilityMap {
    fn cell(&self, y: usize, x: usize) -> u8 {
        u8::from(self[y][x])
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

impl Index<usize> for VisibilityMap {
    type Output = [bool];

    fn index(&self, index: usize) -> &Self::Output {
        let start = index * self.width;
        &self.map[start..start + self.width]
    }
}

impl IndexMut<usize> for VisibilityMap {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let start = index * self.width;
        &mut self.map[start..start + self.width]
    }
}

fn main() {
    let height_map = HeightMap::new(aoc::read_input_lines());
    height_map.visualize();

    let part1 = part1(&height_map);
    let part2 = part2(&height_map);
    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}

fn visibility<'a, I, F>(selected: &u8, indexes: I, value: F) -> usize
where
    I: Iterator<Item = usize>,
    F: Fn(usize) -> &'a u8,
{
    indexes.take_until(|&x| value(x) >= selected).count()
}

fn part2(height_map: &HeightMap) -> usize {
    let (height, width) = (height_map.height, height_map.width);

    (1..height - 1)
        .cartesian_product(1..width - 1)
        .map(|(j, i)| {
            let selected = &height_map[j][i];

            let counter_right = visibility(selected, (i + 1)..width, |x| &height_map[j][x]);
            let counter_left = visibility(selected, (0..i).rev(), |x| &height_map[j][x]);
            let counter_down = visibility(selected, (j + 1)..height, |y| &height_map[y][i]);
            let counter_up = visibility(selected, (0..j).rev(), |y| &height_map[y][i]);

            counter_down * counter_right * counter_up * counter_left
        })
        .max()
        .unwrap()
}

fn part1(height_map: &HeightMap) -> u32 {
    let (height, width) = (height_map.height, height_map.width);
    let mut visible_map = VisibilityMap::new(height, width);

    visible_map.visualize();

    for y in 1..height - 1 {
        let mut highest = height_map[y][0];
        for x in 1..width - 1 {
            check_highest(&height_map[y][x], &mut highest, || visible_map.set(x, y));
        }
    }

    for y in 1..height - 1 {
        let mut highest = height_map[y][width - 1];
        for x in (1..width - 1).rev() {
            check_highest(&height_map[y][x], &mut highest, || visible_map.set(x, y));
        }
    }

    for x in 1..width - 1 {
        let mut highest = height_map[0][x];
        for y in 1..height - 1 {
            check_highest(&height_map[y][x], &mut highest, || visible_map.set(x, y));
        }
    }

    for x in 1..width - 1 {
        let mut highest = height_map[height - 1][x];
        for y in (1..height - 1).rev() {
            check_highest(&height_map[y][x], &mut highest, || visible_map.set(x, y));
        }
    }

    visible_map.visualize();

    visible_map.count()
}

fn check_highest<F>(current: &u8, highest: &mut u8, then: F)
where
    F: FnOnce() -> (),
{
    if current > highest {
        *highest = *current;
        then();
    }
}
