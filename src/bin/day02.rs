use std::env;
use aoc22::read_data;

const DAY: &str = "day02";

fn parse(input: &str) -> impl Iterator<Item = (char, char)> + '_ {
    input.lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().unwrap();
            chars.next();
            let b = chars.next().unwrap();
            (a, b)
        })
}

fn score_a(opponent: char, me: char) -> u64 {
    let outcome = (me as i64 - opponent as i64) % 3;
    let value = (me as i64 - 'X' as i64 + 1) as u64;
    match outcome {
        1 => value,
        2 => 3 + value,
        0 => 6 + value,
        _ => panic!("This does not happen!")
    }
}

fn run_a(input: &str) -> u64 {
    parse(input).map(|(a, b)| score_a(a, b)).sum()
}

fn score_b(opponent: char, result: char) -> u64 {
    let value = (opponent as u64 + result as u64 + 2) % 3 + 1;
    match result {
        'X' => value,
        'Y' => 3 + value,
        'Z' => 6 + value,
        _ => panic!("This does not happen!")
    }
}

fn run_b(input: &str) -> u64 {
    parse(input).map(|(a, b)| score_b(a, b)).sum()
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
mod day02_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 15);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 12);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 11841);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 13022);
    }
}
