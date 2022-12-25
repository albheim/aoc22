use std::env;
use aoc22::read_data;

const DAY: &str = "day25";

fn run_a(input: &str) -> String {
    let mut num = input.lines()
        .map(|line| {
            line.chars().rev().enumerate()
                .map(|(i, c)| {
                    let p = i128::pow(5, i as u32);
                    match c {
                        '2' => 2 * p,
                        '1' => p,
                        '0' => 0,
                        '-' => -p,
                        '=' => -2 * p,
                        _ => panic!("Invalid input"),
                    }
                }).sum::<i128>()
        }).sum();
    let mut ans = String::new();
    let chars = ['=', '-', '0', '1', '2'];
    let mut p: i128 = 1;
    while (p-1)/2 < num {
        p *= 5;
    }
    while p > 1 {
        p /= 5;
        let mut part = 2 * p;
        while (num - part).abs() > (p - 1) / 2 {
            part -= p;
        }
        num -= part;
        ans.push(chars[(part / p + 2) as usize]);
    }
    ans
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = read_data(DAY);

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("No version b today"),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day25_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, "2=-1=0");
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, "2-0=11=-0-2-1==1=-22");
    }
}
