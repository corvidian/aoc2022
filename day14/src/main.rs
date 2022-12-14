use itertools::Itertools;
use std::fmt;
use std::ops::{Index, IndexMut};

// (y, x)
#[derive(Clone, PartialEq)]
struct Position(usize, usize);

impl TryFrom<&str> for Position {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        if let Some((Ok(x), Ok(y))) = s
            .trim()
            .split_once(',')
            .map(|(x, y)| (x.parse(), y.parse()))
        {
            Ok(Position(y, x))
        } else {
            Err("Invalid coordinates")
        }
    }
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({},{})", self.0, self.1))
    }
}

struct Shape(Vec<Position>);

impl TryFrom<&str> for Shape {
    type Error = &'static str;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let result = s
            .trim()
            .split("->")
            .map(Position::try_from)
            .collect::<Result<Vec<_>, _>>();
        result.map(Shape)
    }
}

impl fmt::Display for Shape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0.iter().join(" -> "))
    }
}

trait Visualize {
    fn cell(&self, y: usize, x: usize) -> char;
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

struct CaveMap {
    height: usize,
    width: usize,
    max_y: usize,
    min_x: usize,
    max_x: usize,
    source: Position,
    data: Vec<u8>,
}

impl CaveMap {
    fn new(max_y: usize, min_x: usize, max_x: usize) -> Self {
        let height = max_y + 3;
        let min_x = (500 - height).min(min_x - 1);
        let max_x = (500 + height).max(max_x + 1);
        let width = max_x - min_x + 1;
        CaveMap {
            height,
            width,
            max_y,
            min_x,
            max_x,
            source: Position(0, 500),
            data: vec![b'.'; width * height],
        }
    }

    fn get(&self, y: i32, x: i32) -> Option<&u8> {
        if y < 0
            || x < 0
            || x < self.min_x as i32
            || x > self.max_x as i32
            || y >= self.height as i32
        {
            None
        } else {
            Some(&self.data[y as usize * self.width + x as usize - self.min_x])
        }
    }

    fn set(&mut self, y: usize, x: usize) -> &mut u8 {
        &mut self.data[y * self.width + x - self.min_x]
    }

    fn horizontal_line(&mut self, row: usize, start: usize, end: usize) {
        for x in start..=end {
            *self.set(row, x) = b'#';
        }
    }

    fn vertical_line(&mut self, column: usize, start: usize, end: usize) {
        for y in start..=end {
            *self.set(y, column) = b'#';
        }
    }

    fn drop_one_sand(&mut self) -> bool {
        fn next_from(map: &CaveMap, pos: &Position) -> Option<Position> {
            const CANDIDATES: [(i32, i32); 3] = [(1, 0), (1, -1), (1, 1)];
            CANDIDATES
                .iter()
                .map(|(dy, dx)| (pos.0 as i32 + dy, pos.1 as i32 + dx))
                .filter_map(|(y, x)| {
                    map.get(y, x)
                        .filter(|&c| c == &b'.')
                        .map(|_| Position(y as usize, x as usize))
                })
                .next()
        }

        let mut sand = self.source.clone();
        if self[&sand] == b'o' {
            return false;
        }

        while let Some(next) = next_from(self, &sand) {
            sand = next;
        }
        if sand.0 == self.height - 1 {
            return false;
        }

        self[&sand] = b'o';
        true
    }
}

impl From<Vec<Shape>> for CaveMap {
    fn from(shapes: Vec<Shape>) -> Self {
        let (min_y, max_y) = shapes
            .iter()
            .flat_map(|shape| shape.0.iter().map(|position| position.0))
            .minmax()
            .into_option()
            .expect("No elements");
        let (min_x, max_x) = shapes
            .iter()
            .flat_map(|shape| shape.0.iter().map(|position| position.1))
            .minmax()
            .into_option()
            .expect("No elements");
        dbg!(&min_y, &max_y, &min_x, &max_x);

        let mut map = CaveMap::new(max_y, min_x, max_x);
        *map.set(map.source.0, map.source.1) = b'+';

        for shape in shapes {
            for pair in shape.0.windows(2) {
                match (
                    pair[0].0 as i32 - pair[1].0 as i32,
                    pair[0].1 as i32 - pair[1].1 as i32,
                ) {
                    (0, 0) => {}
                    (0, x) if x < 0 => map.horizontal_line(pair[0].0, pair[0].1, pair[1].1),
                    (0, x) if x > 0 => map.horizontal_line(pair[0].0, pair[1].1, pair[0].1),
                    (y, 0) if y < 0 => map.vertical_line(pair[0].1, pair[0].0, pair[1].0),
                    (y, 0) if y > 0 => map.vertical_line(pair[0].1, pair[1].0, pair[0].0),
                    (_, _) => {
                        unimplemented!("Diagonal lines not supported: {}, {}", pair[0], pair[1])
                    }
                }
            }
        }

        map
    }
}

impl Index<&Position> for CaveMap {
    type Output = u8;

    fn index(&self, index: &Position) -> &Self::Output {
        self.get(index.0 as i32, index.1 as i32).unwrap()
    }
}

impl IndexMut<&Position> for CaveMap {
    fn index_mut(&mut self, index: &Position) -> &mut Self::Output {
        self.set(index.0, index.1)
    }
}

impl Visualize for CaveMap {
    fn cell(&self, y: usize, x: usize) -> char {
        self.data[y * self.width + x].into()
    }

    fn height(&self) -> usize {
        self.height
    }

    fn width(&self) -> usize {
        self.width
    }
}

fn main() {
    let shapes: Vec<Shape> = aoc::input_lines()
        .map(|s| s.as_str().try_into())
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    let mut map: CaveMap = shapes.into();
    map.visualize();

    let mut i = 0;
    while map.drop_one_sand() {
        i += 1;
    }
    map.visualize();
    let part1 = i;

    map.horizontal_line(map.max_y + 2, map.min_x, map.max_x);
    map.visualize();

    while map.drop_one_sand() {
        i += 1;
    }
    map.visualize();

    println!("Part 1: {part1}");
    println!("Part 2: {i}");
}
