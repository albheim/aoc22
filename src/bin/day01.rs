use std::{env, fs};
use partial_sort::PartialSort;

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

    let input = fs::read_to_string("data/day01.txt")
        .expect("Should have been able to read the file");

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("Answer: {}", run_b(&input)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day01_tests {
    use super::*;

    #[test]
    fn test_a() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let result = run_a(input);
        assert_eq!(result, 24000)
    }

    #[test]
    fn test_b() {
        let input = "1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000";
        let result = run_b(input);
        assert_eq!(result, 45000)
    }

    #[test]
    fn real_a() {
        let input = fs::read_to_string("data/day01.txt")
            .expect("Should have been able to read the file");
        let result = run_a(&input);
        assert_eq!(result, 70296)
    }

    #[test]
    fn real_b() {
        let input = fs::read_to_string("data/day01.txt")
            .expect("Should have been able to read the file");
        let result = run_b(&input);
        assert_eq!(result, 205381)
    }
}