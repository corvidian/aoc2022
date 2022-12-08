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
    println!();

    part1(height, width, &height_map);
    part2(height, width, &height_map);
}

fn part2(height: usize, width: usize, height_map: &[Vec<u8>]) {
    let mut prettiest: u32 =0;
    for j in 1..height - 1 {
        for i in 1..width - 1 {
            let selected = height_map[j][i];
            println!("Looking at [{j}][{i}] = {}", selected);

            let mut x = i + 1;
            let mut counter_right = 0;
            while x < width && height_map[j][x] < selected {
                println!("{} {} {} {}", height_map[j][x], selected, x, width);
                counter_right += 1;
                x += 1;
            }
            if x < width {
                counter_right += 1
            }
            println!("to the right: {counter_right}");

            x = i - 1;
            let mut counter_left = 0;
            while height_map[j][x] < selected {
                println!("{} {} {} {}", height_map[j][x], selected, x, width);
                counter_left += 1;
                if x == 0 {
                    break;
                }
                x -= 1;
            }
            if x > 0 {
                counter_left += 1
            }
            println!("to the left: {counter_left}");

            let mut y = j + 1;
            let mut counter_down = 0;
            while y < height && height_map[y][i] < selected {
                println!("{} {} {} {}", height_map[y][i], selected, y, height);
                counter_down += 1;
                y += 1;
            }
            if y < height {
                counter_down += 1
            }
            println!("down: {counter_down}");

            y = j - 1;
            let mut counter_up = 0;
            while height_map[y][i] < selected {
                println!("{} {} {} {}", height_map[y][i], selected, y, height);
                counter_up += 1;
                if y == 0 {
                    break;
                }
                y -= 1;
            }
            if y > 0 {
                counter_up += 1
            }
            println!("up: {counter_up}");

            let score = counter_down * counter_right * counter_up * counter_left;
            println!("score: {score}");
            if prettiest < score {prettiest = score}
        }
    }
    println!("Part 2: {prettiest}")
}

fn part1(height: usize, width: usize, height_map: &[Vec<u8>]) {
    let mut visible_map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 0..width {
        visible_map[0][i] = true;
        visible_map[height - 1][i] = true;
    }

    for i in 0..height {
        visible_map[i][0] = true;
        visible_map[i][width - 1] = true;
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

    let part1: u32 = visible_map
        .iter()
        .map(|line| line.iter().map(|c| u32::from(*c)).sum::<u32>())
        .sum();
    println!("Part1: {part1}");
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
    map.iter().for_each(|line| draw_line_height(line))
}

fn draw_line_height(line: &[u8]) {
    line.iter().for_each(|c| print!("{c}"));
    println!()
}
