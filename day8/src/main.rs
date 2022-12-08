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

    let mut visible_map: Vec<Vec<bool>> = vec![vec![false; width]; height];

    for i in 0..width {
        visible_map[0][i] = true;
        visible_map[height - 1][i] = true;
    }

    for i in 0..height {
        visible_map[i][0] = true;
        visible_map[i][width - 1] = true;
    }

    let mut highest = 0u8;
    for y in 1..height - 1 {
        highest = height_map[y][0];        
        for x in 1..width-1 {
            if height_map[y][x] > highest  {
                highest = height_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    //visualize_visibility(&visible_map);
    //println!();

    for y in 1..height - 1 {
        highest = height_map[y][width-1];        
        for x in 2..width {
            if height_map[y][width-x] > highest  {
                highest = height_map[y][width-x];
                visible_map[y][width-x] = true;
            }
        }
    }

    //visualize_visibility(&visible_map);
    //println!();

    for x in 1..width - 1 {
        highest = height_map[0][x];        
        for y in 1..height-1 {
            if height_map[y][x] > highest  {
                highest = height_map[y][x];
                visible_map[y][x] = true;
            }
        }
    }

    //visualize_visibility(&visible_map);
    //println!();

    for x in 1..width - 1 {
        highest = height_map[height-1][x];  
        println!("highest: {highest}");
        for y in 2..height {
            println!("{} {} {}",height-y,x,height_map[height-y][x]);
            if height_map[height-y][x] > highest  {
                highest = height_map[height-y][x];
                visible_map[height-y][x] = true;
            }
        }
    }

    visualize_visibility(&visible_map);
    println!();

    let part1:u32 = visible_map.iter().map(|line| line.iter().map(|c| if *c {1u32} else {0u32}).sum::<u32>()).sum();
println!("Part1: {part1}");

}

fn visualize_visibility(map: &[Vec<bool>]) {
    map.iter().for_each(|line| draw_line_visibility(&line))
}

fn draw_line_visibility(line: &[bool]) {
    line.iter().for_each(|c| {
        if *c {
            print!("1")
        } else {
            print!("0")
        };
    });
    println!()
}

fn visualize_height(map: &[Vec<u8>]) {
    map.iter().for_each(|line| draw_line_height(&line))
}

fn draw_line_height(line: &[u8]) {
    line.iter().for_each(|c| print!("{c}"));
    println!()
}
