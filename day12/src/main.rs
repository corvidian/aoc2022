use std::collections::VecDeque;

type Position = (usize, usize);

fn main() {
    let input = aoc::read_input_lines();
    let mut map = input
        .iter()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts: Vec<Position> = map
        .iter()
        .enumerate()
        .filter_map(|(i, line)| {
            line.iter()
                .position(|c| *c == 'S' || *c == 'a')
                .map(|pos| (i, pos))
        })
        .collect();

    let min = starts
        .iter()
        .map(|&start| {
            println!("{start:?}");
            map[start.0][start.1] = 'a';

            find(&map, start)
        })
        .min();
    println!("Part 2: {min:?}");
}

fn find(map: &[Vec<char>], start: Position) -> i32 {
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![vec![i32::MAX; width]; height];
    visited[start.0][start.1] = 0;

    let mut queue: VecDeque<Position> = vec![start].into();

    while let Some(current) = queue.pop_front() {
        let steps = visited[current.0][current.1];
        let current_value = map[current.0][current.1] as u32;
        //println!("current: {current:?}, current_value: {current_value:?} steps: {steps}");
        if map[current.0][current.1] == 'E' {
            println!("Result: {steps}");
            return steps;
        }

        for next in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Ok(row) = usize::try_from(current.0 as i32 + next.0) {
                if let Ok(col) = usize::try_from(current.1 as i32 + next.1) {
                    if row < height && col < width {
                        let mut next_value = map[row][col] as u32;
                        if map[row][col] == 'E' {
                            next_value = 'z' as u32
                        }
                        //println!("next: {next:?} next value: {next_value}");
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
    return 0;
}
