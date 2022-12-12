use std::collections::VecDeque;

fn main() {
    let input = aoc::read_input_lines();
    let mut map = input
        .iter()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let starts: Vec<(usize,usize)> = map
        .iter()
        .enumerate()
        .flat_map(|(i, line)| {
            line.iter()
                .enumerate()
                .filter(|(_, &c)| c == 'S' || c == 'a')
                .map(move |(pos, _)| (i, pos))
        })
        .collect();
    let part1_start = starts
        .iter()
        .find(|&start| map[start.0][start.1] == 'S')
        .unwrap();
    map[part1_start.0][part1_start.1] = 'a';

    let min = starts
        .iter()
        .filter_map(|&start| find(&map, start))
        .min()
        .unwrap();

    println!("Part 1: {:?}", find(&map, *part1_start));
    println!("Part 2: {min:?}");
}

fn find(map: &[Vec<char>], start: (usize,usize)) -> Option<i32> {
    let height = map.len();
    let width = map[0].len();

    let mut visited = vec![vec![i32::MAX; width]; height];
    visited[start.0][start.1] = 0;

    let mut queue: VecDeque<(usize,usize)> = vec![start].into();

    while let Some(current) = queue.pop_front() {
        let steps = visited[current.0][current.1];
        let current_value = map[current.0][current.1] as u32;
        if map[current.0][current.1] == 'E' {
            return Some(steps);
        }

        for delta in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            if let Ok(row) = usize::try_from(current.0 as i32 + delta.0) {
                if let Ok(col) = usize::try_from(current.1 as i32 + delta.1) {
                    if row < height && col < width {
                        let mut next_value = map[row][col] as u32;
                        if map[row][col] == 'E' {
                            next_value = 'z' as u32
                        }
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
    return None;
}
