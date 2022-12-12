use std::collections::VecDeque;

type Position = (usize, usize);

fn main() {
    let input = aoc::read_input_lines();
    let mut map = input
        .iter()
        .map(|i| i.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let height = map.len();
    let width = map[0].len();

    let start: Position = map
        .iter()
        .enumerate()
        .find_map(|(i, line)| line.iter().position(|c| *c == 'S').map(|pos| (i, pos)))
        .unwrap();

    println!("{start:?}");

    let mut visited = vec![vec![i32::MAX; width]; height];
    visited[start.0][start.1] = 0;
    map[start.0][start.1] = 'a';

    let mut queue: VecDeque<Position> = vec![start].into();

    while let Some(current) = queue.pop_front() {
        let steps = visited[current.0][current.1];
        let current_value = map[current.0][current.1] as u32;
        //println!("current: {current:?}, current_value: {current_value:?} steps: {steps}");
        if map[current.0][current.1] == 'E' {
            println!("Result: {steps}");
            return;
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
                        if next_value <= current_value + 1 && visited[row][col] > steps && !queue.contains(&(row,col)){
                            visited[row][col] = steps + 1;
                            queue.push_back((row, col));
                        }
                    }
                }
            }
        }
    }
}
