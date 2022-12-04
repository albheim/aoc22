use std::{env, fs};
use aoc22::split_line;

fn parse(line: &str) -> ((u64, u64), (u64, u64)) {
    let result: Vec<u64> = split_line(line, "{}-{},{}-{}")
        .unwrap()
        .iter()
        .map(|s| s.parse().unwrap())
        .collect();
    
    ((result[0], result[1]), (result[2], result[3]))
}

fn contains(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.0 && a.1 >= b.1
}

fn overlap(a: (u64, u64), b: (u64, u64)) -> bool {
    a.0 <= b.1 && a.1 >= b.0
}

fn run_a(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let (area1, area2) = parse(line);
            contains(area1, area2) || contains(area2, area1)
        }).filter(|a| *a)
        .count() as u64
}

fn run_b(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let (area1, area2) = parse(line);
            overlap(area1, area2)
        }).filter(|a| *a)
        .count() as u64
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("data/day04.txt")
        .expect("Should have been able to read the file");

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("Answer: {}", run_b(&input)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

        let result = run_a(input);

        assert_eq!(result, 2);
    }

    #[test]
    fn test_b() {
        let input = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8";

        let result = run_b(input);

        assert_eq!(result, 4);
    }

    #[test]
    fn real_a() {
        let input = fs::read_to_string("data/day04.txt")
            .expect("Should have been able to read the file");

        let result = run_a(&input);

        assert_eq!(result, 534);
    }

    #[test]
    fn real_b() {
        let input = fs::read_to_string("data/day04.txt")
            .expect("Should have been able to read the file");

        let result = run_b(&input);

        assert_eq!(result, 841);
    }
}