use std::{env, fs};

fn score_a(opponent: i64, me: i64) -> u64 {
    let outcome = (me - opponent) % 3;
    let value = (me - 'X' as i64 + 1) as u64;
    match outcome {
        1 => value,
        2 => 3 + value,
        0 => 6 + value,
        _ => panic!("This does not happen!")
    }
}

fn score_b(opponent: char, result: char) -> u64 {
    let value = (opponent as u64 + result as u64 + 2) % 3 + 1;
    match result {
        'X' => value,
        'Y' => 3 + value,
        'Z' => 6 + value,
        _ => panic!("This does not happen!")
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = args[1].as_str();

    let input = fs::read_to_string("data/day02.txt")
        .expect("Should have been able to read the file");

    let games = input.lines()
        .map(|line| {
            let mut chars = line.chars();
            let a = chars.next().unwrap();
            chars.next();
            let b = chars.next().unwrap();
            (a, b)
        });
        
    match version {
        "a" => println!("{}", games.map(|(a, b)| score_a(a as i64, b as i64)).sum::<u64>()),
        "b" => println!("{}", games.map(|(a, b)| score_b(a, b)).sum::<u64>()),
        _ => panic!("Incorrect input, should be a or b.")
    }
}
