use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

//Example const PART1_ROW: i64 = 10;
//Example const BEACONS_ON_ROW:i64 = 1;
const PART1_ROW: i64 = 2000000;
const BEACONS_ON_ROW: i64 = 1;
const LOWER_LIMIT: i64 = 0;
const UPPER_LIMIT: i64 = 4000000;

lazy_static! {
    static ref RE: Regex =
        Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)")
            .unwrap();
}

fn parse_line(line: &str) -> Sensor {
    let groups: Vec<_> = RE
        .captures(line)
        .expect("Invalid input line")
        .iter()
        .skip(1)
        .map(|x| x.unwrap().as_str().parse::<i64>().unwrap())
        .collect();
    let (x, y, bx, by) = (groups[0], groups[1], groups[2], groups[3]);
    let distance: i64 = (x - bx).abs() + (y - by).abs();
    Sensor { x, y, distance }
}

fn main() {
    let sensors: Vec<_> = aoc::input_lines().map(|line| parse_line(&line)).collect();

    for y in LOWER_LIMIT..=UPPER_LIMIT {
        let mut sections: Vec<_> = sensors
            .iter()
            .filter_map(|sensor| {
                let diff = sensor.distance - (sensor.y - y).abs();
                (diff >= 0).then_some((sensor.x - diff, sensor.x + diff))
            })
            .collect();

        let mut combined_section = sections.pop().unwrap();
        while let Some((i, section)) = sections.iter().enumerate().find(|(_, &section)| {
            (combined_section.0 <= section.0 && section.0 <= combined_section.1)
                || (combined_section.0 <= section.1 && section.1 <= combined_section.1)
                || (section.0 <= combined_section.0 && combined_section.0 <= section.1)
                || (section.0 <= combined_section.1 && combined_section.1 <= section.1)
        }) {
            combined_section.0 = combined_section.0.min(section.0);
            combined_section.1 = combined_section.1.max(section.1);
            sections.remove(i);
        }

        if y == PART1_ROW {
            println!(
                "Part 1: {}",
                combined_section.1 - combined_section.0 + 1 - BEACONS_ON_ROW
            );
        }

        if !sections.is_empty() {
            println!("Combined: {combined_section:?}");
            println!("Left over: {sections:?}");
            sections.push(combined_section);

            for x in LOWER_LIMIT..=UPPER_LIMIT {
                match sections.iter().find(|(min, max)| *min <= x && x <= *max) {
                    Some(_) => {}
                    None => {
                        println!("Found: {x}, {y}, Frequency = {}", x * 4000000 + y);
                        return;
                    }
                };
            }
        }
    }
}
