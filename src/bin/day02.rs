use std::{env, fs};

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

    let input = fs::read_to_string("data/day02.txt")
        .expect("Should have been able to read the file");

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("Answer: {}", run_b(&input)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day02_tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "A Y\nB X\nC Z";
        let result: Vec<u64> = parse(input).map(|(a, b)| score_a(a, b)).collect();

        assert_eq!(result, vec![8, 1, 6])
    }

    #[test]
    fn test_b() {
        let input = "A Y\nB X\nC Z";
        let result: Vec<u64> = parse(input).map(|(a, b)| score_b(a, b)).collect();

        assert_eq!(result, vec![4, 1, 7])
    }

    #[test]
    fn real_a() {
        let input = fs::read_to_string("data/day02.txt")
            .expect("Should have been able to read the file");
        let result = run_a(&input);

        assert_eq!(result, 11841)
    }

    #[test]
    fn real_b() {
        let input = fs::read_to_string("data/day02.txt")
            .expect("Should have been able to read the file");
        let result = run_b(&input);

        assert_eq!(result, 13022)
    }
}