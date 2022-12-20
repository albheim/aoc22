use std::env;
use aoc22::read_data;

const DAY: &str = "day20";

fn run_a(input: &str) -> isize {
    let mut values: Vec<(usize, isize)> = input.lines().map(|line| line.parse().unwrap()).enumerate().collect();
    let mut order: Vec<usize> = (0..values.len()).collect();

    for i in 0..order.len() {
        let mut idx = order[i];
        let val = values[idx].1;
        let mut steps = val.abs();
        let step = val.signum();
        while steps > 0 {
            let nextidx = (idx as isize + step).rem_euclid(values.len() as isize) as usize;
            values.swap(idx, nextidx);
            
            order[values[idx].0] = (order[values[idx].0] as isize - step).rem_euclid(values.len() as isize) as usize;
            order[values[nextidx].0] = (order[values[nextidx].0] as isize + step).rem_euclid(values.len() as isize) as usize;
            idx = nextidx;
            steps -= 1;
        }
    }

    let zeroidx = values.iter().position(|v| v.1 == 0).unwrap();
    values.rotate_left(zeroidx);
    [1000, 2000, 3000].iter().map(|i| {
        let idx = i % values.len();
        values[idx].1
    }).sum()
}

fn run_b(input: &str) -> isize {
    let mut values: Vec<(usize, isize)> = input.lines().map(|line| 811589153 * line.parse::<isize>().unwrap()).enumerate().collect();
    let mut order: Vec<usize> = (0..values.len()).collect();

    for _ in 0..10 {
        for i in 0..order.len() {
            let mut idx = order[i];
            let val = values[idx].1;
            let mut steps = val.abs() % (values.len() - 1) as isize;
            let step = val.signum();
            while steps > 0 {
                let nextidx = (idx as isize + step).rem_euclid(values.len() as isize) as usize;
                values.swap(idx, nextidx);
                
                order[values[idx].0] = (order[values[idx].0] as isize - step).rem_euclid(values.len() as isize) as usize;
                order[values[nextidx].0] = (order[values[nextidx].0] as isize + step).rem_euclid(values.len() as isize) as usize;
                idx = nextidx;
                steps -= 1;
            }
        }
    }

    let zeroidx = values.iter().position(|v| v.1 == 0).unwrap();
    values.rotate_left(zeroidx);
    [1000, 2000, 3000].iter().map(|i| {
        let idx = i % values.len();
        values[idx].1
    }).sum()
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
mod day20_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 3);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 9687);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 1623178306);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 13520);
    }
}
