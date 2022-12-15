use std::{env, ops::Range};
use aoc22::read_data;
use parseline::parseln;
use std::collections::HashSet;
use std::cmp;

const DAY: &str = "day15";

#[derive(Eq, PartialEq, Hash, Clone, Copy, Debug)]
struct Point {
    x: i64, 
    y: i64,
}

impl Point {
    fn l1_dist(&self, other: &Self) -> i64 {
        i64::abs(self.x - other.x) + i64::abs(self.y - other.y)
    }
}

#[derive(Debug)]
struct RangeSet {
    ranges: Vec<Range<i64>>
}

impl RangeSet {
    fn insert(mut self, mut other: RangeSet) -> RangeSet{
        while let Some(mut range) = other.ranges.pop() {
            let mut i = 0;
            while i < self.ranges.len() {
                if self.ranges[i].start <= range.end && range.start <= self.ranges[i].end {
                    let tmp = self.ranges.swap_remove(i);
                    let combined = cmp::min(tmp.start, range.start)..cmp::max(tmp.end, range.end);
                    range = combined;
                } else {
                    i += 1;
                }
            }
            self.ranges.push(range);
        }
        self
    }

    fn count(&self) -> usize {
        self.ranges.iter().map(|range| cmp::max(0, range.end - range.start) as usize).sum()
    }
}

struct Sensor {
    pos: Point,
    closest_beacon: Point,
    dist: i64,
}

impl Sensor {
    fn build(input: &str) -> Vec<Sensor> {
        input.lines()
            .map(|line| {
                parseln!(line, "Sensor at x={}, y={}: closest beacon is at x={}, y={}",
                    x1: i64, y1: i64, x2: i64, y2: i64);
                let pos = Point{ x: x1, y: y1 };
                let closest_beacon = Point{ x: x2, y: y2 };
                //println!("pos {:?} beacon {:?} range {}", pos, closest_beacon, pos.l1_dist(&closest_beacon));
                Sensor {
                    pos,
                    closest_beacon,
                    dist: pos.l1_dist(&closest_beacon),
                }
            }).collect()
    }

    fn contains(&self, p: &Point) -> bool {
        self.pos.l1_dist(p) <= self.dist
    }
}

fn run_a(input: &str, y: i64) -> usize {
    let sensors = Sensor::build(input);
    
    let sensor_coverage = sensors.iter()
        .map(|sensor| {
            let mid = sensor.pos.x;
            let range = sensor.dist - i64::abs(sensor.pos.y - y);
            RangeSet{ ranges: vec![mid-range..mid+range+1] }
        }).reduce(|a, b| a.insert(b)).unwrap();

    let beacons = sensors.iter()
        .map(|sensor| sensor.closest_beacon)
        .filter(|beacon| beacon.y == y)
        .collect::<HashSet<_>>().len();

    sensor_coverage.count() - beacons
}

fn contains(sensors: &[Sensor], p: Point) -> bool {
    sensors.iter().all(|sens| !sens.contains(&p))
}

fn run_b(input: &str, upper: i64) -> i64 {
    let sensors = Sensor::build(input);

    if contains(&sensors, Point{ x: upper, y: 0 }) {
        return upper * 4000000
    }
    for sens in sensors.iter() {
        let mut p = sens.pos;
        p.x -= sens.dist + 1;
        if p.x < 0 {
            p.y -= p.x;
            p.x = 0;
        }
        if p.y < 0 {
            p.x -= p.y;
            p.y = 0;
        }
        let dist = i64::min(sens.dist + 1, upper - i64::max(p.x, p.y));
        for _ in 0..=dist {
            if contains(&sensors, p) {
                return p.x * 4000000 + p.y
            }
            p.x += 1;
            p.y += 1;
        }
    }
    panic!("Shouldn't happen");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = read_data(DAY);

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input, 2000000)),
        "b" => println!("Answer: {}", run_b(&input, 4000000)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day08_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY), 10);
        assert_eq!(result, 26);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY), 2000000);
        assert_eq!(result, 6275922);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY), 20);
        assert_eq!(result, 56000011);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY), 4000000);
        assert_eq!(result, 11747175442119);
    }
}
