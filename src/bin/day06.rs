use std::{env, fs};

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

    let input = fs::read_to_string("data/day06.txt")
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
        assert_eq!(run_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(run_a("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(run_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(run_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_b() {
        assert_eq!(run_a("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(run_a("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(run_a("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(run_a("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(run_a("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn real_a() {
        let input = fs::read_to_string("data/day06.txt")
            .expect("Should have been able to read the file");

        let result = run_a(&input);

        assert_eq!(result, 1766);
    }

    #[test]
    fn real_b() {
        let input = fs::read_to_string("data/day06.txt")
            .expect("Should have been able to read the file");

        let result = run_b(&input);

        assert_eq!(result, 2383);
    }
}