use std::env;
use aoc22::read_data;

const DAY: &str = "day03";

fn parse(line: &str) -> Vec<u64> {
    line.chars().map(priority).collect()
}

fn priority(c: char) -> u64 {
    if c.is_lowercase() {
        (c as i64 - 'a' as i64 + 1) as u64
    } else {
        (c as i64 - 'A' as i64 + 27) as u64
    }
}

fn overlap(a: &[u64], b: &[u64]) -> Vec<u64> {
    a.iter().filter(|i| b.contains(i)).cloned().collect()
}

fn run_a(input: &str) -> u64 {
    input.lines()
        .map(|line| {
            let backpack = parse(line);
            let halflen = backpack.len()/2;
            let overlap = overlap(&backpack[..halflen], &backpack[halflen..]);
            overlap[0]
        }).sum()
}

fn run_b(input: &str) -> u64 {
    let backpacks: Vec<Vec<u64>> = input.lines()
        .map(parse).collect();

    let mut sum = 0;
    for i in 0..backpacks.len()/3 {
        sum += overlap(&backpacks[3*i], &overlap(&backpacks[3*i+1], &backpacks[3*i+2]))[0];
    }
    sum
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = read_data(DAY);

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("Answer: {}", run_b(&input)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day03_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 157);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 70);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 8252);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 2828);
    }
}
