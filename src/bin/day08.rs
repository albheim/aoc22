use std::env;
use aoc22::read_data;

const DAY: &str = "day08";

fn parse(input: &str) -> Vec<Vec<i64>> {
    input.lines()
        .map(|line| line.chars().map(|c| c as i64 - '0' as i64).collect())
        .collect()
}

fn run_a(input: &str) -> usize {
    let trees = parse(input);
    let s = trees.len();
    let mut visible = vec![vec![false; s]; s];
    for i in 0..s {
        let mut rmax = -1;
        let mut lmax = -1;
        let mut umax = -1;
        let mut dmax = -1;
        for j in 0..s {
            if lmax < trees[i][j] {
                lmax = trees[i][j];
                visible[i][j] = true;
            } 
            if rmax < trees[i][s - j - 1] {
                rmax = trees[i][s - j - 1];
                visible[i][s - j - 1] = true;
            } 
            if umax < trees[j][i] {
                umax = trees[j][i];
                visible[j][i] = true;
            } 
            if dmax < trees[s - j - 1][i] {
                dmax = trees[s - j - 1][i];
                visible[s - j - 1][i] = true;
            } 
        }
    }
    visible.iter().map(|row| row.iter().filter(|&x| *x).count()).sum()
}

fn run_b(input: &str) -> usize {
    let trees = parse(input);
    let s = trees.len();
    let mut max = 0;
    for y in 1..s-1 {
        for x in 1..s-1 {
            let mut dxneg = 1;
            while x > dxneg && trees[y][x-dxneg] < trees[y][x] {
                dxneg += 1;
            }
            let mut dxpos = 1;
            while x + dxpos + 1 < s && trees[y][x+dxpos] < trees[y][x] {
                dxpos += 1;
            }
            let mut dyneg = 1;
            while y > dyneg && trees[y-dyneg][x] < trees[y][x] {
                dyneg += 1;
            }
            let mut dypos = 1;
            while y + dypos + 1 < s && trees[y+dypos][x] < trees[y][x] {
                dypos += 1;
            }
            let tot = dxneg * dxpos * dyneg * dypos;
            if max < tot {
                max = tot;
            }
        }
    }
    max
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
        assert_eq!(result, 21);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 8);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 1684);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 486540);
    }
}
