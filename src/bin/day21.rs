use std::env;
use aoc22::read_data;
use parseline::parseln;
use std::collections::HashMap;

const DAY: &str = "day21";

#[derive(Debug, Copy, Clone)]
struct Op {
    op: char,
    left: Option<isize>,
    right: Option<isize>,
}

fn name2id(name: &str) -> usize {
    let mut id = 0;
    for c in name.chars() {
        id = (id << 8) + c as usize;
    }
    id
}

fn id2name(id: usize) -> String {
    let mut name = String::new();
    for i in 0..4 {
        name.push(((id >> (8 * i)) & ((1 << 8) - 1)) as u8 as char)
    }
    name.chars().rev().collect()
}

fn calc(op: char, left: isize, right: isize) -> isize {
    match op {
        '+' => left + right,
        '-' => left - right,
        '*' => left * right,
        '/' => left / right,
        _ => panic!("Invalid operator")
    }
}

fn propagate(rest: &mut HashMap<usize, Op>, parents: &HashMap<usize, (usize, bool)>, id: usize, value: isize) -> Option<isize> {
    //println!("Propagating name {} id {} and value {}", id2name(id), id, value);
    //println!("{:?}", rest);
    if id == name2id("root") {
        return Some(value);
    }
    let &(tid, right) = parents.get(&id).unwrap();
    let mut e = rest.remove(&tid).unwrap();
    if right {
        match e.left {
            Some(x) => {
                let a = calc(e.op, x, value);
                return propagate(rest, parents, tid, a);
            },
            None => {
                e.right = Some(value);
                rest.insert(tid, e);
            },
        }
    } else {
        match e.right {
            Some(x) => {
                let a = calc(e.op, value, x);
                return propagate(rest, parents, tid, a);
            },
            None => {
                e.left = Some(value);
                rest.insert(tid, e);
            },
        }
    }
    //println!("Updated {}: {:?}", tid, e);
    None
}

fn run_a(input: &str) -> isize {
    let mut ready = Vec::new();
    let mut rest = HashMap::new();
    let mut parents = HashMap::new();
    for line in input.lines() {
        if line.len() < 17 {
            parseln!(line, "{}: {}", name: String, number: isize);
            ready.push((name2id(&name), number));
        } else {
            parseln!(line, "{}: {} {} {}", name: String, left: String, op: char, right: String);
            let name = name2id(&name);
            let left = name2id(&left);
            let right = name2id(&right);
            rest.insert(name, Op { op, left: None, right: None });
            parents.insert(left, (name, false));
            parents.insert(right, (name, true));
        }
    }

    for &(id, value) in ready.iter() {
        if let Some(x) = propagate(&mut rest, &parents, id, value) {
            return x
        }
    }
    0
}

fn run_b(input: &str) -> isize {
    let mut ready = Vec::new();
    let mut rest = HashMap::new();
    let mut parents = HashMap::new();
    let mut children = HashMap::new();
    for line in input.lines() {
        if &line[..4] == "humn" {
            // Skip
        } else if &line[..4] == "root" {
            parseln!(line, "{}: {} {} {}", name: String, left: String, _op: char, right: String);
            let id = name2id(&name);
            let left = name2id(&left);
            let right = name2id(&right);
            rest.insert(id, Op { op: '=', left: None, right: None });
            parents.insert(left, (id, false));
            parents.insert(right, (id, true));
            children.insert(id, (left, right));
        } else if line.len() < 17 {
            parseln!(line, "{}: {}", name: String, number: isize);
            ready.push((name2id(&name), number));
        } else {
            parseln!(line, "{}: {} {} {}", name: String, left: String, op: char, right: String);
            let id = name2id(&name);
            let left = name2id(&left);
            let right = name2id(&right);
            rest.insert(id, Op { op, left: None, right: None });
            parents.insert(left, (id, false));
            parents.insert(right, (id, true));
            children.insert(id, (left, right));
        }
    }

    for &(id, value) in ready.iter() {
        propagate(&mut rest, &parents, id, value);
    }
    let mut curr = name2id("root");
    let mut pval = 0;
    loop {
        let op = rest.get(&curr).unwrap();
        let &(tleft, tright) = children.get(&curr).unwrap();
        if op.left.is_none() {
            curr = tleft;
        } else {
            curr = tright;
        }
        pval = match *op {
            Op { op: '=', left: None, right: Some(x) } => x,
            Op { op: '=', left: Some(x), right: None } => x,
            Op { op: '+', left: None, right: Some(x) } => pval - x,
            Op { op: '+', left: Some(x), right: None } => pval - x,
            Op { op: '-', left: None, right: Some(x) } => pval + x,
            Op { op: '-', left: Some(x), right: None } => x - pval,
            Op { op: '*', left: None, right: Some(x) } => pval / x,
            Op { op: '*', left: Some(x), right: None } => pval / x,
            Op { op: '/', left: None, right: Some(x) } => pval * x,
            Op { op: '/', left: Some(x), right: None } => x / pval,
            _ => panic!("Should not happen"),
        };
        if curr == name2id("humn") {
            return pval;
        }
    }
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
mod day21_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 152);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 331319379445180);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 301);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 3715799488132);
    }
}
