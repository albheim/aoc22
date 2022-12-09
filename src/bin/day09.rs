use std::env;
use aoc22::read_data;
use parseline::parseln;
use std::collections::HashMap;
use std::cmp;

const DAY: &str = "day09";

struct Rope {
    joints: Vec<(i64, i64)>,
    tail_visits: HashMap<(i64, i64), u64>
}

impl Rope {
    fn new(n: usize) -> Self {
        Rope {
            joints: vec![(0, 0); n],
            tail_visits: HashMap::new(),
        }
    }

    fn step(&mut self, dir: char, n: u64) {
        for _ in 0..n {
            // Move head
            match dir {
                'R' => self.joints[0].0 += 1,
                'L' => self.joints[0].0 -= 1,
                'U' => self.joints[0].1 += 1,
                'D' => self.joints[0].1 -= 1,
                _ => panic!("Wrong input")
            }

            // Follow with remaining joints
            for i in 1..self.joints.len() {
                if cmp::max(i64::abs(self.joints[i-1].0 - self.joints[i].0), i64::abs(self.joints[i-1].1 - self.joints[i].1)) > 1 {
                    let dx = self.joints[i-1].0 - self.joints[i].0;
                    self.joints[i].0 += (dx + i64::signum(dx)) / 2;
                    let dy = self.joints[i-1].1 - self.joints[i].1; 
                    self.joints[i].1 += (dy + i64::signum(dy)) / 2;
                }
            }
            
            // Record tail position
            self.tail_visits.insert(self.joints[self.joints.len()-1], 1);
        }
    }

    fn tail_visits(&self) -> usize {
        self.tail_visits.len()
    }
}

fn run_a(input: &str) -> usize {
    let mut rope = Rope::new(2);
    for line in input.lines() {
        parseln!(line, "{} {}", dir: char, n: u64);
        rope.step(dir, n);
    }
    rope.tail_visits()
}

fn run_b(input: &str) -> usize {
    let mut rope = Rope::new(10);
    for line in input.lines() {
        parseln!(line, "{} {}", dir: char, n: u64);
        rope.step(dir, n);
    }
    rope.tail_visits()
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
mod day08_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 13);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 1);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 5874);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 486540);
    }
}
