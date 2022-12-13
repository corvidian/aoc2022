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

    let part1 = find(&map, &[part1_start], target);
    let part2 = find(&map, &starts, target);
    println!("Part 1: {part1:?}");
    println!("Part 2: {part2:?}");
}

fn find(map: &[Vec<char>], starts: &[(usize, usize)], target: (usize, usize)) -> Option<i32> {
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![vec![i32::MAX; width]; height];
    let mut parents: Vec<Vec<Option<(usize, usize)>>> = vec![vec![None; width]; height];
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(width * height / 10);
    queue.extend(starts.iter());
    for start in starts {
        visited[start.0][start.1] = 0;
    }

    while let Some((y, x)) = queue.pop_front() {
        let steps = visited[y][x];
        if (y, x) == target {
            visualize_path(map, &parents, target);
            return Some(steps);
        }

        for (dy, dx) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Ok(row) = usize::try_from(y as i32 + dy) {
                if let Ok(col) = usize::try_from(x as i32 + dx) {
                    if row < height && col < width {
                        let next_value = map[row][col] as u32;
                        if next_value <= map[y][x] as u32 + 1 && visited[row][col] == i32::MAX {
                            visited[row][col] = steps + 1;
                            parents[row][col] = Some((y, x));
                            queue.push_back((row, col));
                        }
                    }
                }
            }
        }
    }
    None
}

fn visualize_path(
    map: &[Vec<char>],
    parents: &[Vec<Option<(usize, usize)>>],
    target: (usize, usize),
) {
    let mut output: Vec<_> = map.to_vec();
    let mut counter = 0;
    let (mut y, mut x) = target;
    output[y][x] = '⛳';

    while let Some(parent) = parents[y][x] {
        let (py, px) = parent;
        counter += 1;
        if py < y {
            output[py][px] = '⏬';
        } else if py > y {
            output[py][px] = '⏫';
        } else if px < x {
            output[py][px] = '⏩';
        } else if px > x {
            output[py][px] = '⏪';
        }

        (y, x) = parent;
    }

    output[y][x] = '🔰';

    let narrow_a = 'a' as u32;
    let fullwidth_a = '\u{FF21}' as u32;

    for (y, row) in output.iter().enumerate() {
        print!("{y:2} ");
        for char in row {
            let char_value = *char as u32 - narrow_a;
            if char_value < 26 {
                print!("{}", char::try_from(fullwidth_a + char_value).unwrap());
            } else {
                print!("{char}");
            }
        }
        println!();
    }
    print!("   ");
    for i in 0..output[0].len() {
        print!("{}", char::try_from(0xff10u32 + (i % 10) as u32).unwrap());
    }
    print!("\n   ");
    for i in 0..=output[0].len() / 10 {
        print!(
            "{}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}\u{3000}",
            char::try_from(0xff10u32 + i as u32).unwrap()
        );
    }
    println!("\nCounter: {counter}");
    println!();
    println!();
}
