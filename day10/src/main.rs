fn main() {
    let mut cycles = 0u32;
    let mut x = 1i32;
    let mut sum = 0i32;
    
    let input = aoc::read_input_lines();
    let mut iter = input.iter();
    while let Some(line) = iter.next() {
        if line == "noop" {
            cycle(&mut cycles, &x, &mut sum);
        } else if line.starts_with("addx"){
            cycle(&mut cycles, &x, &mut sum);
            cycle(&mut cycles, &x, &mut sum);
            let operand = line.split(' ').last().unwrap().parse::<i32>().unwrap();
            x += operand;
        } else {
            panic!("Unknown instruction {line}");
        }
    }

    println!("Part 1: {sum}");
}

fn cycle(cycles: &mut u32, x: &i32, sum: &mut i32) {
    *cycles += 1;
    if (*cycles + 20) % 40 == 0  {
        println!("{cycles} {x}");
        *sum += *cycles as i32 * *x;
    }
}
