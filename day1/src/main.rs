fn main() {
    aoc::init_logging();

    let mut sums: Vec<u32> = vec![];

    let mut current: u32 = 0;
    for line in aoc::input_lines() {
        if let Ok(number) = line.parse::<u32>() {
            current += number;
        } else {
            sums.push(current);
            current = 0;
        }
        log::debug!("Current: {current}");
    }

    sums.sort_by(|a, b| b.cmp(a));
    log::info!("Highest: {}", sums[0]);
    log::info!(
        "{} + {} + {} = {}",
        sums[0],
        sums[1],
        sums[2],
        sums[0] + sums[1] + sums[2]
    )
}
