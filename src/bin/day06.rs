use std::env;
use aoc22::read_data;

const DAY: &str = "day06";

fn all_different(a: &Vec<char>) -> bool {
    for i in 0..a.len() {
        for j in (i+1)..a.len() {
            if a[i] == a[j] {
                return false
            }
        }
    }
    true
}

fn first_n_equal(input: &str, n: usize) -> u64 {
    let mut buffer: Vec<char> = input.chars().take(n).collect();
    let mut idx_buffer = 0;
    let mut idx_input = n;
    for c in input.chars().skip(n) {
        if all_different(&buffer) {
            return idx_input as u64
        }
        buffer[idx_buffer] = c;
        idx_buffer = (idx_buffer + 1) % n;
        idx_input += 1
    }
    0
}

fn run_a(input: &str) -> u64 {
    first_n_equal(input, 4)
}

fn run_b(input: &str) -> u64 {
    first_n_equal(input, 14)
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
mod day06_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result: Vec<u64> = read_test(DAY).lines().map(run_a).collect();
        assert_eq!(result, vec![5, 6, 10, 11]);
    }

    #[test]
    fn test_b() {
        let result: Vec<u64> = read_test(DAY).lines().map(run_b).collect();
        assert_eq!(result, vec![23, 23, 29, 26]);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 1766);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 2383);
    }
}
