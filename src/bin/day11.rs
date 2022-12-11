use std::env;
use aoc22::read_data;

const DAY: &str = "day11";

enum Operation {
    Add(i64),
    Multiply(i64),
    Square,
}

impl Operation {
    fn apply(&self, value: i64) -> i64 {
        match self {
            Operation::Add(x) => value + x,
            Operation::Multiply(x) => value * x,
            Operation::Square => value * value,
        }
    }
}

struct Monkey {
    items: Vec<i64>,
    operation: Operation,
    throw_rule: i64,
    throw_target_true: usize,
    throw_target_false: usize,
    inspected: usize
}

impl Monkey {
    fn build(input: &str) -> Monkey {
        let lines: Vec<&str> = input.lines().collect();

        let items = lines[1]
            .split(": ").last().unwrap()
            .split(", ").map(|n| n.parse::<i64>().unwrap()).collect();

        let operation: Vec<&str> = lines[2]
            .split(" = ").last().unwrap()
            .split(' ').collect();
        let operation = match (operation[0], operation[1], operation[2]) {
            ("old", "*", "old") => Operation::Square,
            ("old", "*", x) => Operation::Multiply(x.parse().unwrap()),
            ("old", "+", x) => Operation::Add(x.parse().unwrap()),
            x => panic!("Failed parsing monkey {:?}", x),
        };
        
        let divisibility = lines[3]
            .split(' ').last().unwrap()
            .parse().unwrap();
        let case_true = lines[4]
            .split(' ').last().unwrap()
            .parse().unwrap();
        let case_false = lines[5]
            .split(' ').last().unwrap()
            .parse().unwrap();
        
        Monkey {
            items,
            operation,
            throw_rule: divisibility,
            throw_target_true: case_true, 
            throw_target_false: case_false,
            inspected: 0,
        }
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

fn gcd(a: i64, b: i64) -> i64 {
    let (mut min, mut max) = if a < b {
        (a, b)
    } else {
        (b, a)
    };
    
    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }
        max = min;
        min = res;
    }
}

fn run_a(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::build).collect();

    for _ in 0..20 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let worry_level = monkeys[i].items[j];
                let new_worry_level = monkeys[i].operation.apply(worry_level) / 3;
                let target = if new_worry_level % monkeys[i].throw_rule == 0 {
                    monkeys[i].throw_target_true
                } else {
                    monkeys[i].throw_target_false
                };
                monkeys[target].items.push(new_worry_level);
            }
            monkeys[i].inspected += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
    let mut inspect: Vec<usize> = monkeys.iter().map(|mk| mk.inspected).collect();
    inspect.sort_by(|a,b| b.partial_cmp(a).unwrap());
    inspect[0] * inspect[1]
}

fn run_b(input: &str) -> usize {
    let mut monkeys: Vec<Monkey> = input.split("\n\n").map(Monkey::build).collect();

    let lcm_divider = monkeys.iter().map(|m| m.throw_rule).reduce(lcm).unwrap();

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            for j in 0..monkeys[i].items.len() {
                let worry_level = monkeys[i].items[j];
                let new_worry_level = monkeys[i].operation.apply(worry_level) % lcm_divider;
                let target = if new_worry_level % monkeys[i].throw_rule == 0 {
                    monkeys[i].throw_target_true
                } else {
                    monkeys[i].throw_target_false
                };
                monkeys[target].items.push(new_worry_level);
            }
            monkeys[i].inspected += monkeys[i].items.len();
            monkeys[i].items.clear();
        }
    }
    let mut inspect: Vec<usize> = monkeys.iter().map(|mk| mk.inspected).collect();
    inspect.sort_by(|a,b| b.partial_cmp(a).unwrap());
    inspect[0] * inspect[1]
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
        assert_eq!(result, 10605);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 55458);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 2713310158);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 0);
    }
}
