use lazy_static::lazy_static;
use regex::Regex;
use itertools::Itertools;

#[derive (Debug)]
struct Sensor {
    x: i64,
    y: i64,
    distance: i64,
}

//const ROW: i64 = 10;
//const BEACONS_ON_ROW:i64 = 1;
const ROW: i64 = 2000000;
const BEACONS_ON_ROW:i64 = 1;

lazy_static! {
    static ref RE: Regex = Regex::new(r"Sensor at x=(-?\d+), y=(-?\d+): closest beacon is at x=(-?\d+), y=(-?\d+)").unwrap();
}

fn parse_line(line: &str) -> Sensor {
    let groups: Vec<_> = RE.captures(line).expect("Invalid input line").iter().skip(1).map(|x| x.unwrap().as_str().parse::<i64>().unwrap()).collect();
    let (x,y,bx,by) = (groups[0],groups[1],groups[2],groups[3]);    
    let distance: i64 = (x-bx).abs() + (y-by).abs();
    println!("{x},{y},{bx},{by},{},{},{distance}", (x-bx).abs() , (y-by).abs());
    Sensor {x,y,distance}
}

fn main() {
    let sensors: Vec<_> = aoc::input_lines().map(|line| parse_line(&line)).collect();
    for sensor in &sensors {
        println!("{sensor:?}")
    }    
    let min_x = sensors.iter().map(|sensor| sensor.x - sensor.distance).min().unwrap()-2;
    let max_x = sensors.iter().map(|sensor| sensor.x + sensor.distance).max().unwrap()+2;
    println!("min {min_x}, max {max_x}");

    let mut found = 0;
    
    for x in min_x..=max_x {
        match sensors.iter().find(|sensor| ((sensor.x - x).abs() + (sensor.y - ROW).abs()) <= sensor.distance) {
            Some(sensor) => found  += 1,
            None => {},
        };
    }
    found -= BEACONS_ON_ROW;

    println!("Part 1: {found}");
}
