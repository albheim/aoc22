use std::{env, fmt::Display};
use aoc22::read_data;

const DAY: &str = "day17";

struct Shape {
    tiles: Vec<Vec<bool>>
}

impl Shape {
    fn setup() -> Vec<Shape> {
        vec![
            Shape {
                tiles: vec![vec![true]; 4]
            },
            Shape {
                tiles: vec![
                    vec![false, true, false],
                    vec![true, true, true],
                    vec![false, true, false],
                ]
            },
            Shape {
                tiles: vec![
                    vec![true, false, false],
                    vec![true, false, false],
                    vec![true, true, true],
                ]
            },
            Shape {
                tiles: vec![vec![true; 4]]
            },
            Shape {
                tiles: vec![vec![true; 2]; 2]
            },
        ]
    }
}

struct Chamber {
    columns: Vec<Vec<bool>>,
    jets: Vec<i64>,
    jet_counter: usize,
    highest: usize,
}

impl Display for Chamber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in (0..self.columns[0].len()).rev() {
            write!(f, "|")?;
            for x in 0..7 {
                if self.columns[x][y] {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f, "|")?;
        }
        write!(f, "+-------+")
    }
}

impl Chamber {
    fn setup(input: &str) -> Chamber {
        let columns = vec![vec![false; 7]; 7];
        let jets = input.chars().map(|c| if c == '<' {-1} else {1}).collect();

        Chamber { 
            columns, 
            jets, 
            jet_counter: 0,
            highest: 0,
        }
    }

    fn check_overlap(&self, pos: (usize, usize), stone: &Shape) -> bool {
        //println!("Checking overlap for stone with shape {:?} at pos {:?}", stone.tiles, pos);
        for x in 0..stone.tiles.len() {
            for y in 0..stone.tiles[x].len() {
                if self.columns[x + pos.0][y + pos.1] && stone.tiles[x][y] {
                    return true
                }
            }
        }
        false
    }

    fn drop_stone(&mut self, stone: &Shape) {
        let mut pos: (usize, usize) = (2, self.highest + 3);
        loop {
            //println!("Curr pos {:?} wind coming {}", pos, self.jets[self.jet_counter % self.jets.len()]);
            let newx = pos.0 as i64 + self.jets[self.jet_counter % self.jets.len()];
            self.jet_counter += 1;
            //println!("newx {newx}");
            if newx >= 0 && newx as usize + stone.tiles.len() < 8 && !self.check_overlap((newx as usize, pos.1), stone) {
                pos.0 = newx as usize;
            }
            if pos.1 == 0 {
                break
            }
            pos.1 -= 1;
            if self.check_overlap(pos, stone) {
                pos.1 += 1;
                break
            }
        }

        let mut newhigh = self.highest;
        for x in 0..stone.tiles.len() {
            for y in 0..stone.tiles[x].len() {
                if stone.tiles[x][y] {
                    self.columns[x + pos.0][y + pos.1] = stone.tiles[x][y];
                    newhigh = newhigh.max(y + pos.1 + 1);
                }
            }
        }
        if newhigh != self.highest {
            for _ in self.highest..newhigh {
                for x in 0..self.columns.len() {
                    self.columns[x].push(false);
                }
            }
            self.highest = newhigh;
        }
    }
}

fn run_a(input: &str) -> usize {
    let mut chamber = Chamber::setup(input);
    let shapes = Shape::setup();
    for i in 0..2022 {
        chamber.drop_stone(&shapes[i % 5]);
    }
    
    chamber.highest
}

fn check_recurring(hist: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
    let lastidx = hist.len() - 1;
    let mut i = lastidx;
    loop {
        if i == 0 || 2 * i < hist.len() {
            return None
        }
        i -= 1;
        if hist[i] == hist[lastidx] {
            let n = lastidx - i;
            if i < n {
                return None
            } else if hist[i-n+1..=i] == hist[lastidx-n+1..] {
                return Some((i-n+1, n))
            }
        }
    }
}

fn run_b(input: &str, depth: usize) -> usize {
    let mut chamber = Chamber::setup(input);
    let shapes = Shape::setup();
    let mut hist = Vec::new();
    let mut heights = Vec::new();
    let mut i = 0;
    loop {
        chamber.drop_stone(&shapes[i % 5]);
        hist.push((i % 5, chamber.jet_counter % chamber.jets.len()));
        heights.push(chamber.highest);

        if let Some((first_recur, recur_len)) = check_recurring(&hist) {
            let num_recurs = (depth - (first_recur + 1)) / recur_len;
            let recur_height = heights[first_recur+recur_len] - heights[first_recur];
            let last_stones = (depth - (first_recur + 1)) % recur_len;
            let last_height = heights[first_recur+last_stones] - heights[first_recur];
            return heights[first_recur] + num_recurs * recur_height + last_height;
        }

        i += 1;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let version = &args[1];

    let input = read_data(DAY);

    match version.as_str() {
        "a" => println!("Answer: {}", run_a(&input)),
        "b" => println!("Answer: {}", run_b(&input, 1000000000000)),
        _ => panic!("Incorrect input, should be a or b.")
    }
}

#[cfg(test)]
mod day17_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 3068);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 3055);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY), 1000000000000);
        assert_eq!(result, 1514285714288);
    }

    #[test]
    fn test_b_on_atest() {
        let result = run_b(&read_test(DAY), 2022);
        assert_eq!(result, 3068);
    }

    #[test]
    fn test_b_on_areal() {
        let result = run_b(&read_data(DAY), 2022);
        assert_eq!(result, 3055);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY), 1000000000000);
        assert_eq!(result, 1507692307690);
    }
}
