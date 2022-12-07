use std::env;
use aoc22::{split_line, read_data};

const DAY: &str = "day05";

struct CrateStacks {
    stacks: Vec<Vec<char>>
}

impl CrateStacks {
    fn from(lines: Vec<&str>) -> Self {
        let n = (lines.last().unwrap().chars().count() + 1) / 4;
        let mut stacks: Vec<Vec<char>> = (0..n).map(|_| Vec::new()).collect();

        for line in &lines[1..] {
            let mut chunk_iter = line.split('[');
            let mut idx = chunk_iter.next().unwrap().chars().count() / 4;
            for chunk in chunk_iter {
                stacks[idx].push(chunk.chars().next().unwrap());
                idx += (chunk.chars().count() + 1) / 4;
            }
        }

        CrateStacks { stacks }
    }

    fn rearrange_9000(&mut self, from: usize, to: usize, n: usize) {
        for _ in 0..n {
            let a = self.stacks[from].pop().unwrap();
            self.stacks[to].push(a);
        }
    }

    fn rearrange_9001(&mut self, from: usize, to: usize, n: usize) {
        let mut tmp = Vec::new();
        for _ in 0..n {
            tmp.push(self.stacks[from].pop().unwrap());
        }
        for _ in 0..n {
            self.stacks[to].push(tmp.pop().unwrap());
        }
    }

    fn toplayer(&self) -> String {
        self.stacks.iter().map(|stack| stack.last().unwrap()).collect()
    }
}

fn run_a(input: &str) -> String {
    let line_iter = input.lines();
    let mut crate_inputs: Vec<&str> = line_iter.take_while(|line| !line.is_empty()).collect();
    crate_inputs.reverse();
    let mut crates = CrateStacks::from(crate_inputs);

    let commands = input.lines().skip_while(|line| !line.is_empty()).skip(1);
    for line in commands {
        let a: Vec<usize> = split_line(line, "move {} from {} to {}")
            .unwrap()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        crates.rearrange_9000(a[1] - 1, a[2] - 1, a[0])
    }

    crates.toplayer()
}

fn run_b(input: &str) -> String {
    let line_iter = input.lines();
    let mut crate_inputs: Vec<&str> = line_iter.take_while(|line| !line.is_empty()).collect();
    crate_inputs.reverse();
    let mut crates = CrateStacks::from(crate_inputs);

    let commands = input.lines().skip_while(|line| !line.is_empty()).skip(1);
    for line in commands {
        let a: Vec<usize> = split_line(line, "move {} from {} to {}")
            .unwrap()
            .into_iter()
            .map(|s| s.parse().unwrap())
            .collect();
        crates.rearrange_9001(a[1] - 1, a[2] - 1, a[0])
    }

    crates.toplayer()
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
mod day05_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, "CMZ");
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, "MCD");
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, "ZBDRNPMVH");
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, "WDLPFNNNB");
    }
}
