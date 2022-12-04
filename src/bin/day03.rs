use std::{env, fs};

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

    let input = fs::read_to_string("data/day03.txt")
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
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let result: Vec<u64> = input.lines()
            .map(|line| {
                let backpack = parse(line);
                let halflen = backpack.len()/2+1;
                let overlap = overlap(&backpack[..halflen], &backpack[halflen..]);
                overlap[0]
            }).collect();

        assert_eq!(result, vec![16, 38, 42, 22, 20, 19])
    }

    #[test]
    fn test_b() {
        let input = "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        let backpacks: Vec<Vec<u64>> = input.lines()
            .map(parse).collect();

        let mut result = Vec::new();
        for i in 0..backpacks.len()/3 {
            let isec = overlap(&backpacks[3*i+1], &backpacks[3*i+2]);
            let isec = overlap(&backpacks[3*i], &isec);
            result.push(isec[0])
        }

        assert_eq!(result, vec![18, 52])
    }

    #[test]
    fn test_priority() {
        let prios: Vec<u64> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"
            .chars().map(priority).collect();
        
        let ans: Vec<u64> = (1..53).collect();

        assert_eq!(prios, ans)
    }

    #[test]
    fn real_a() {
        let input = fs::read_to_string("data/day03.txt")
            .expect("Should have been able to read the file");

        let result = run_a(&input);

        assert_eq!(result, 8252);
    }

    #[test]
    fn real_b() {
        let input = fs::read_to_string("data/day03.txt")
            .expect("Should have been able to read the file");

        let result = run_b(&input);

        assert_eq!(result, 2828);
    }
}