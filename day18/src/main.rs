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

fn find_air(start: Position, visited: &mut [Vec<Vec<bool>>], space: &[Vec<Vec<bool>>]) -> bool {
    for a in &mut *visited {
        for b in a {
            for c in b {
                *c = false;
            }
        }
    }

    let found = visit(start, visited, space);
    found
}

fn visit(next: Position, visited: &mut [Vec<Vec<bool>>], space: &[Vec<Vec<bool>>]) -> bool {
    let (x, y, z) = next;
    let end = space.len() - 1;
    if x == 0 || x == end || y == 0 || y == end || z == 0 || z == end {
        return true;
    }
    visited[next.0][next.1][next.2] = true;

    for (dx, dy, dz) in DELTAS {
        let (x, y, z) = (c(&x, dx), c(&y, dy), c(&z, dz));
        if space[x][y][z] && !visited[x][y][z] && visit((x, y, z), visited, space) {
            return true;
        }
    }
    false
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
    let mut open_air = vec![vec![vec![false; dimensions]; dimensions]; dimensions];
    for x in 0..dimensions {
        for y in 0..dimensions {
            for z in 0..dimensions {
                if x == 0 || x == dimensions-1 || y == 0 || y == dimensions-1 || z == 0 || z == dimensions-1 {
                    open_air[x][y][z] = true;
                }            
            }
        }
    }
    let mut visited = vec![vec![vec![false; dimensions]; dimensions]; dimensions];

    for (x, y, z) in &cubes {
        space[*x][*y][*z] = false;
    }

    let mut free_sides = 0u64;

    for (x, y, z) in &cubes {
        for (dx, dy, dz) in DELTAS {
            let (x, y, z) = (c(x, dx), c(y, dy), c(z, dz));
            if space[x][y][z] && (open_air[x][y][z] || find_air((x, y, z), &mut visited, &space)) {
                open_air[x][y][z] = true;
                free_sides += 1;
            }
        }
    }

    println!("{free_sides}");
}
