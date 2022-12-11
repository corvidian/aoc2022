fn main() {
    let mut cycles = 0u32;
    let mut x = 1i32;
    let mut sum = 0i32;

    aoc::read_input_lines().iter().for_each(|line| {
        if line == "noop" {
            cycle(&mut cycles, &x, &mut sum);
        } else if line.starts_with("addx") {
            cycle(&mut cycles, &x, &mut sum);
            cycle(&mut cycles, &x, &mut sum);
            let operand = line.split(' ').last().unwrap().parse::<i32>().unwrap();
            x += operand;
        } else {
            panic!("Unknown instruction {line}");
        }
    });

    println!("Part 1: {sum}");
}

fn cycle(cycles: &mut u32, x: &i32, sum: &mut i32) {
    let pixel = ((*cycles) % 40) as i32;
    if (pixel - *x).abs() <= 1 {
        print!("#")
    } else {
        print!(".");
    }

    *cycles += 1;
    if *cycles % 40 == 0 {
        println!()
    }
    if (*cycles + 20) % 40 == 0 {
        *sum += *cycles as i32 * *x;
    }
}
