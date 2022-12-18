use std::collections::VecDeque;

type Position = (usize, usize, usize);

static DELTAS: [(i64, i64, i64); 6] = [
    (1, 0, 0),
    (-1, 0, 0),
    (0, 1, 0),
    (0, -1, 0),
    (0, 0, 1),
    (0, 0, -1),
];

fn parse_line(line: String) -> Position {
    let split = line
        .split(',')
        .map(|s| s.parse().unwrap())
        .collect::<Vec<usize>>();
    (split[0] + 1, split[1] + 1, split[2] + 1)
}

fn c(a: &usize, da: i64) -> usize {
    (*a as i64 + da) as usize
}

fn coords(
    x: usize,
    dx: i64,
    y: usize,
    dy: i64,
    z: usize,
    dz: i64,
    end: i64,
) -> Option<(usize, usize, usize)> {
    let cx = x as i64 + dx;
    let cy = y as i64 + dy;
    let cz = z as i64 + dz;
    if cx >= 0 && cx <= end && cy >= 0 && cy <= end && cz >= 0 && cz <= end {
        Some((cx as usize, cy as usize, cz as usize))
    } else {
        None
    }
}

fn find_air(space: &[Vec<Vec<bool>>], open_air: &mut [Vec<Vec<bool>>]) {
    let mut queue: VecDeque<(usize, usize, usize)> =
        VecDeque::with_capacity(space.len() * space.len() * space.len() / 10);
    let end = (space.len() - 1) as i64;
    queue.push_back((0, 0, 0));
    open_air[0][0][0] = true;

    while let Some((x, y, z)) = queue.pop_front() {
        for (dy, dx, dz) in DELTAS {
            if let Some((x, y, z)) = coords(x, dx, y, dy, z, dz, end) {
                if !open_air[x][y][z] && space[x][y][z] {
                    open_air[x][y][z] = true;
                    queue.push_back((x, y, z));
                }
            }
        }
    }
}

fn main() {
    let cubes = aoc::input_lines().map(parse_line).collect::<Vec<_>>();

    let dimensions = cubes
        .iter()
        .map(|cube| cube.0.max(cube.1.max(cube.2)))
        .max()
        .unwrap()
        + 2;

    println!("Max dimension: {dimensions}");

    let mut space = vec![vec![vec![true; dimensions]; dimensions]; dimensions];
    for (x, y, z) in &cubes {
        space[*x][*y][*z] = false;
    }

    let mut open_air = vec![vec![vec![false; dimensions]; dimensions]; dimensions];
    find_air(&space, &mut open_air);

    let mut part1 = 0u64;
    let mut part2 = 0u64;

    for (x, y, z) in &cubes {
        for (dx, dy, dz) in DELTAS {
            let (x, y, z) = (c(x, dx), c(y, dy), c(z, dz));
            if space[x][y][z] {
                part1 += 1;
            }
            if open_air[x][y][z] {
                part2 += 1;
            }
        }
    }

    println!("Part 1: {part1}");
    println!("Part 2: {part2}");
}
