use std::{env, fmt::Display};
use aoc22::read_data;
use std::collections::HashMap;

const DAY: &str = "day23";

enum Direction {
    N, W, E, S
}

struct Map {
    tiles: Vec<Vec<bool>>,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                write!(f, "{}", if self.tiles[i][j] { '#' } else { '.' })?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let tiles: Vec<Vec<bool>> = input.lines()
            .map(|line| line.chars().map(|c| c == '#').collect())
            .collect();
        Map { tiles }
    }

    fn is_occupied(&self, i: isize, j: isize) -> bool {
        i >= 0 && (i as usize) < self.tiles.len() && 
        j >= 0 && (j as usize) < self.tiles[0].len() && 
        self.tiles[i as usize][j as usize]
    }

    fn step(&mut self, dirs: &Vec<Direction>) -> bool {
        let mut propositions = HashMap::new();
        for i in 0..self.tiles.len() {
            for j in 0..self.tiles[0].len() {
                if self.tiles[i][j] {
                    let (i, j) = (i as isize, j as isize);
                    let (nw, n, ne) = if i == 0 {
                        (false, false, false)
                    } else {
                        (self.is_occupied(i-1, j-1), self.is_occupied(i-1, j), self.is_occupied(i-1, j+1))
                    };
                    let (sw, s, se) = if i == self.tiles.len() as isize - 1 {
                        (false, false, false)
                    } else {
                        (self.is_occupied(i+1, j-1), self.is_occupied(i+1, j), self.is_occupied(i+1, j+1))
                    };
                    let w = self.is_occupied(i, j-1);
                    let e = self.is_occupied(i, j+1);
                    if !n && !ne && !nw && !s && !se && !sw && !w && !e {
                        continue
                    }

                    for dir in dirs {
                        match dir {
                            Direction::N => if !n && !ne && !nw {
                                propositions.entry((i-1, j)).and_modify(|e| *e = (-1, -1)).or_insert((i, j));
                                break
                            },
                            Direction::S => if !s && !se && !sw {
                                propositions.entry((i+1, j)).and_modify(|e| *e = (-1, -1)).or_insert((i, j));
                                break
                            },
                            Direction::W => if !nw && !w && !sw {
                                propositions.entry((i, j-1)).and_modify(|e| *e = (-1, -1)).or_insert((i, j));
                                break
                            },
                            Direction::E => if !ne && !e && !se {
                                propositions.entry((i, j+1)).and_modify(|e| *e = (-1, -1)).or_insert((i, j));
                                break
                            },
                        }
                    }
                }
            }
        }
        let mut offset = (0, 0);
        let mut moved = false;
        for (to, from) in propositions {
            if from != (-1, -1) {
                moved = true;
                if to.0 + offset.0 < 0 {
                    self.tiles.insert(0, vec![false; self.tiles[0].len()]);
                    offset.0 += 1;
                }
                if (to.0 + offset.0) as usize >= self.tiles.len() {
                    self.tiles.push(vec![false; self.tiles[0].len()]);
                }
                if to.1 + offset.1 < 0 {
                    for i in 0..self.tiles.len() {
                        self.tiles[i].insert(0, false);
                    }
                    offset.1 += 1;
                }
                if (to.1 + offset.1) as usize >= self.tiles[0].len() {
                    for i in 0..self.tiles.len() {
                        self.tiles[i].push(false);
                    }
                }
                //println!("from {:?} to {:?} offset {:?}", from, to, offset);
                self.tiles[(from.0 + offset.0) as usize][(from.1 + offset.1) as usize] = false;
                self.tiles[(to.0 + offset.0) as usize][(to.1 + offset.1) as usize] = true;
            }
        }
        moved
    }
}

fn run_a(input: &str) -> usize {
    let mut map = Map::parse(input);
    //println!("Initial state:\n{map}");
    let mut dirs = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    for _ in 0..10 {
        map.step(&dirs);
        dirs.rotate_left(1);
        //println!("{map}");
    }
    let mut elves = 0;
    let mut lower = (map.tiles.len(), map.tiles[0].len());
    let mut higher = (0, 0);
    for i in 0..map.tiles.len() {
        for j in 0..map.tiles[0].len() {
            if map.tiles[i][j] {
                lower.0 = lower.0.min(i);
                lower.1 = lower.1.min(j);
                higher.0 = higher.0.max(i);
                higher.1 = higher.1.max(j);
                elves += 1;
            }
        }
    }
    //println!("{:?} {:?} {}", lower, higher, elves);
    (higher.0 - lower.0 + 1) * (higher.1 - lower.1 + 1) - elves
}

fn run_b(input: &str) -> isize {
    let mut map = Map::parse(input);
    let mut dirs = vec![Direction::N, Direction::S, Direction::W, Direction::E];
    let mut round = 1;
    loop {
        if !map.step(&dirs) {
            return round
        }
        round += 1;
        dirs.rotate_left(1);
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
mod day23_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 110);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 4114);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 20);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 970);
    }
}
