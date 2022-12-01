use std::{env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = fs::read_to_string("data/day01.txt")
        .expect("Should have been able to read the file");
        
    let mut total_carry: Vec<u64> = vec![0];

    input.lines().for_each(|x| {
        if x.is_empty() {
            total_carry.push(0);
        } else {
            let n = total_carry.len() - 1;
            total_carry[n] += x.parse::<u64>().unwrap();
        }
    });

    match version.as_str() {
        "a" => println!("{}", total_carry.iter().max().unwrap()),
        "b" => {
            total_carry.sort_by(|a, b| b.cmp(a));
            println!("{:?}", total_carry[0..3].iter().sum::<u64>())
        },
        _ => panic!("Incorrect input, should be a or b.")
    }
}
