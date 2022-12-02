use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let mut sums: Vec<u32> = vec![];

    if let Ok(lines) = read_lines("./input.txt") {
        let mut current: u32 = 0;

        for line in lines {
            if let Ok(Ok(number)) = line.map(|s| s.parse::<u32>()) {
                current += number;
            } else {
                sums.push(current);
                current = 0;
            }
            println!("Current: {current}");
        }
    } else {
        println!("Error reading file")
    }

    sums.sort_by(|a, b| b.cmp(a));
    println!("Highest: {}", sums[0]);
    println!(
        "{} + {} + {} = {}",
        sums[0],
        sums[1],
        sums[2],
        sums[0] + sums[1] + sums[2]
    )
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
