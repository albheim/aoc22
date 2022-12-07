use std::env;
use aoc22::read_data;
use partial_sort::PartialSort;

const DAY: &str = "day01";

fn parse(input: &str) -> Vec<u64> {
    let mut total_carry: Vec<u64> = vec![0];

    input.lines().for_each(|x| {
        if x.is_empty() {
            total_carry.push(0);
        } else {
            let n = total_carry.len() - 1;
            total_carry[n] += x.parse::<u64>().unwrap();
        }
    });

    total_carry
}

fn sum_n_largest(total_carry: &mut Vec<u64>, n: usize) -> u64 {
    total_carry.partial_sort(n, |a, b| b.cmp(a));
    total_carry.iter().take(n).sum::<u64>()
}

fn run_a(input: &str) -> u64 {
    let mut total_carry = parse(input);
    sum_n_largest(&mut total_carry, 1)
}

fn run_b(input: &str) -> u64 {
    let mut total_carry = parse(input);
    sum_n_largest(&mut total_carry, 3)
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
mod day01_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 24000);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 45000);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 70296);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 205381);
    }
}
