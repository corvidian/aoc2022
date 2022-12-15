use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

type Section = (i64,i64);

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

fn parse() -> Vec<Sensor> {
    aoc::input_lines().map(|line| parse_line(&line)).collect()
}

fn get_sections(y: i64, sensors: &[Sensor], sections: &mut Vec<Section>) {
    sensors
        .iter()
        .filter_map(|sensor| {
            let diff = sensor.distance - (sensor.y - y).abs();
            (diff >= 0).then_some((sensor.x - diff, sensor.x + diff))
        })
        .for_each(|section| sections.push(section));
}

fn combine_sections(sections: &mut Vec<Section>) -> Section {
    let mut combined_section = sections.pop().unwrap();
    while let Some((i, section)) = sections.iter().enumerate().find(|(_, &section)| {
        (combined_section.0 <= section.0 && section.0 <= combined_section.1)
            || (combined_section.0 <= section.1 && section.1 <= combined_section.1)
            || (section.0 <= combined_section.0 && combined_section.0 <= section.1)
            || (section.0 <= combined_section.1 && combined_section.1 <= section.1)
    }) {
        combined_section.0 = combined_section.0.min(section.0);
        combined_section.1 = combined_section.1.max(section.1);
        sections.swap_remove(i);
    }
    combined_section
}

fn main() {
    use std::time::Instant;
    let start_time = Instant::now();
    let sensors: Vec<_> = parse();
    println!("Parsing done: {:?}", start_time.elapsed());

    let mut sections: Vec<Section> = Vec::with_capacity(sensors.len());

    for y in LOWER_LIMIT..=UPPER_LIMIT {
        get_sections(y, &sensors, &mut sections);

        let combined_section = combine_sections(&mut sections);

        if y == PART1_ROW {
            println!(
                "Part 1: {}",
                combined_section.1 - combined_section.0 + 1 - BEACONS_ON_ROW
            );
        }

        if !sections.is_empty() {
            println!("Found row: {:?}", start_time.elapsed());
            let left_over_combined = combine_sections(&mut sections);
            assert!(sections.is_empty());
            let x = if left_over_combined.1 < combined_section.0 {
                left_over_combined.1 + 1
            } else {
                combined_section.1 + 1
            };
            println!("Found: {x}, {y}, Frequency = {}", x * 4000000 + y);
            println!("Found frequency: {:?}", start_time.elapsed());
            return;
        }
    }
}
