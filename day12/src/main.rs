use std::collections::VecDeque;

fn main() {
    let input = aoc::read_input_lines();
    let mut map = input
        .iter()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut starts: Vec<(usize, usize)> = vec![];
    let mut part1_start = (0, 0);
    let mut target = (0, 0);

    for (y, row) in map.iter_mut().enumerate() {
        for (x, char) in row.iter_mut().enumerate() {
            match char {
                'S' => {
                    part1_start = (y, x);
                    starts.push((y, x));
                    *char = 'a';
                }
                'E' => {
                    target = (y, x);
                    *char = 'z';
                }
                'a' => starts.push((y, x)),
                _ => {}
            }
        }
    }

    let min = starts
        .iter()
        .filter_map(|&start| find(&map, start, target, false))
        .min();

    println!("Part 1: {:?}", find(&map, part1_start, target, true));
    println!("Part 2: {min:?}");
}

fn find(
    map: &[Vec<char>],
    start: (usize, usize),
    target: (usize, usize),
    visualize: bool,
) -> Option<i32> {
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![vec![i32::MAX; width]; height];
    visited[start.0][start.1] = 0;

    let mut queue: VecDeque<(usize, usize)> = vec![start].into();

    while let Some((x, y)) = queue.pop_front() {
        let steps = visited[x][y];
        if (x, y) == target {
            if visualize {
                visualize_visited(map, &visited, target);
            }
            return Some(steps);
        }
        let current_value = map[x][y] as u32;

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Ok(row) = usize::try_from(x as i32 + dx) {
                if let Ok(col) = usize::try_from(y as i32 + dy) {
                    if row < height && col < width {
                        let next_value = map[row][col] as u32;
                        if next_value <= current_value + 1
                            && visited[row][col] > steps
                            && !queue.contains(&(row, col))
                        {
                            visited[row][col] = steps + 1;
                            queue.push_back((row, col));
                        }
                    }
                }
            }
        }
    }
    None
}

fn visualize_visited(map: &[Vec<char>], visited: &[Vec<i32>], target: (usize, usize)) {
    let mut output: Vec<_> = map.to_vec();

    let (mut y, mut x) = target;
    output[y][x] = '🔰';

    while visited[y][x] > 0 {
        if y > 0 && visited[y - 1][x] as u32 == visited[y][x] as u32 - 1 {
            output[y - 1][x] = '⏬';
            y -= 1;
        } else if x > 0 && visited[y][x - 1] as u32 == visited[y][x] as u32 - 1 {
            output[y][x - 1] = '⏩';
            x -= 1;
        } else if y < visited.len() && visited[y + 1][x] as u32 == visited[y][x] as u32 - 1 {
            output[y + 1][x] = '⏫';
            y += 1;
        } else if x < visited[y].len() && visited[y][x + 1] as u32 == visited[y][x] as u32 - 1 {
            output[y][x + 1] = '⏪';
            x += 1;
        }
    }

    output[y][x] = '⛳';

    let narrow_a = 'a' as u32;
    let fullwidth_a = '\u{FF21}' as u32;

    for row in output {
        for char in row {
            let char_value = char as u32 - narrow_a;
            if char_value < 26 {
                print!("{}", char::try_from(fullwidth_a + char_value).unwrap());
            } else {
                print!("{char}");
            }
        }
        println!();
    }

    println!();
    println!();
}
