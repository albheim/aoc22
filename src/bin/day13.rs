use std::env;
use aoc22::read_data;
use std::cmp::Ordering;

const DAY: &str = "day13";

#[derive(Eq, PartialEq, Debug)]
enum Packet {
    Sub(Vec<Packet>),
    Val(i64),
}

impl Packet {
    fn build(input: &str) -> Packet {
        let chars: Vec<char> = input.chars().collect();
        Packet::parse_sub(&chars[..], &mut 0)
    }

    fn parse_sub(part: &[char], idx: &mut usize) -> Packet {
        let mut contents = Vec::new();
        let mut acc: Option<i64> = None;
        
        loop {
            *idx += 1;
            match part[*idx] {
                '0'..='9' => acc = Some(
                    match acc {
                        Some(x) => 10 * x,
                        None => 0,
                    } + (part[*idx] as i64 - '0' as i64)
                ),
                '[' => contents.push(Packet::parse_sub(part, idx)),
                ']' => {
                    if let Some(x) = acc {
                        contents.push(Packet::Val(x));
                    }
                    return Packet::Sub(contents)
                },
                ',' => if let Some(x) = acc {
                    contents.push(Packet::Val(x));
                    acc = None;
                },
                _ => panic!("Invalid character encountered.")
            }
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Packet::Val(x), Packet::Val(y)) => x.cmp(y),
            (Packet::Sub(x), Packet::Sub(y)) => x.cmp(y),
            (Packet::Val(x), y) => Packet::Sub(vec![Packet::Val(*x)]).cmp(y),
            (x, Packet::Val(y)) => x.cmp(&Packet::Sub(vec![Packet::Val(*y)])),
        }
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn run_a(input: &str) -> usize {
    input.split("\n\n")
        .map(|pair| {
            let mut itr = pair.lines();
            let a = Packet::build(itr.next().unwrap());
            let b = Packet::build(itr.next().unwrap());
            (a, b)
        }).enumerate()
        .filter(|(_, (a, b))| a < b)
        .map(|(i, _)| i + 1).sum()
}

fn run_b(input: &str) -> usize {
    let divider1 = Packet::Sub(vec![Packet::Sub(vec![Packet::Val(2)])]);
    let divider2 = Packet::Sub(vec![Packet::Sub(vec![Packet::Val(6)])]);
    
    let packets: Vec<Packet> = input.lines()
        .filter(|line| !line.is_empty())
        .map(Packet::build)
        .collect();
    
    let idx1 = packets.iter().filter(|&packet| packet < &divider1).count();
    let idx2 = packets.iter().filter(|&packet| packet < &divider2).count();

    (idx1 + 1) * (idx2 + 2)
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
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 5625);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 140);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 23111);
    }
}
