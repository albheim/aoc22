use std::{env, fmt::Display};
use aoc22::read_data;
use std::collections::{VecDeque, HashSet};
use std::hash::{Hash, Hasher};

const DAY: &str = "day24";

#[derive(Debug, Clone, Copy)]
enum Direction { N, S, W, E }

#[derive(Debug)]
struct Blizzard {
    pos: [usize; 2],
    dir: Direction,
}

#[derive(Debug)]
struct Map {
    tiles: Vec<Vec<bool>>,
    blizzards: Vec<Blizzard>,
    time: usize,
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut tiles:Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; self.tiles[0].len()]; self.tiles.len()];
        for blizzard in self.blizzards.iter() {
            tiles[blizzard.pos[0]][blizzard.pos[1]].push(blizzard.dir)
        }
        writeln!(f, "#.######")?;
        for i in 1..tiles.len()-1 {
            write!(f, "#")?;
            for j in 1..tiles[0].len()-1 {
                if tiles[i][j].is_empty() {
                    write!(f, ".")? 
                } else if tiles[i][j].len() > 1 { 
                    write!(f, "{}", tiles[i][j].len())?
                } else {
                    write!(f, "{}", match tiles[i][j][0] {
                        Direction::N => "^",
                        Direction::S => "v",
                        Direction::W => "<",
                        Direction::E => ">",
                    })?;
                }
            }
            writeln!(f, "#")?;
        }
        writeln!(f, "######.#")?;
        Ok(())
    }
}

impl Map {
    fn parse(input: &str) -> Self {
        let mut tiles = Vec::new();
        let mut blizzards = Vec::new();
        for line in input.lines() {
            let mut row = Vec::new();
            for c in line.chars() {
                match c {
                    '#' => row.push(false),
                    '.' => row.push(true),
                    '^' => {
                        blizzards.push(Blizzard { 
                            pos: [tiles.len(), row.len()], 
                            dir: Direction::N 
                        });
                        row.push(false);
                    },
                    '<' => {
                        blizzards.push(Blizzard { 
                            pos: [tiles.len(), row.len()], 
                            dir: Direction::W 
                        });
                        row.push(false);
                    },
                    'v' => {
                        blizzards.push(Blizzard { 
                            pos: [tiles.len(), row.len()], 
                            dir: Direction::S 
                        });
                        row.push(false);
                    },
                    '>' => {
                        blizzards.push(Blizzard { 
                            pos: [tiles.len(), row.len()], 
                            dir: Direction::E 
                        });
                        row.push(false);
                    },
                    _ => panic!("Invalid input")
                }
            }
            tiles.push(row);
        }
        Map { tiles, blizzards, time: 0 }
    }

    fn step(&mut self) {
        // Reset middle
        for i in 1..self.tiles.len()-1 {
            for j in 1..self.tiles[0].len()-1 {
                self.tiles[i][j] = true;
            }
        }
        // Move blizzard and tag unmovable locations
        for blizzard in self.blizzards.iter_mut() {
            match blizzard.dir {
                Direction::N => {
                    blizzard.pos[0] -= 1;
                    if blizzard.pos[0] == 0 {
                        blizzard.pos[0] = self.tiles.len() - 2;
                    }
                },
                Direction::S => {
                    blizzard.pos[0] += 1;
                    if blizzard.pos[0] == self.tiles.len() - 1 {
                        blizzard.pos[0] = 1;
                    }
                },
                Direction::W => {
                    blizzard.pos[1] -= 1;
                    if blizzard.pos[1] == 0 {
                        blizzard.pos[1] = self.tiles[0].len() - 2;
                    }
                },
                Direction::E => {
                    blizzard.pos[1] += 1;
                    if blizzard.pos[1] == self.tiles[0].len() - 1 {
                        blizzard.pos[1] = 1;
                    }
                },
            }
            self.tiles[blizzard.pos[0]][blizzard.pos[1]] = false;
        }
        // Update time
        self.time += 1;
    }

    fn walk(&mut self, from: [usize; 2], to: [usize; 2]) {
        let mut states = VecDeque::from([State {
            pos: from,
            time: self.time,
        }]);
        let mut visited = HashSet::new();

        while let Some(state) = states.pop_front() {
            if state.pos == to {
                break
            }
            if state.time == self.time {
                self.step();
                //println!("State {:?} + Queue {:?}", (state.pos, state.time), states.iter().map(|s| (s.pos, s.time)).collect::<Vec<([usize; 2], usize)>>());
                //println!("Minute {}\n{}", self.time, self);
            } else if visited.contains(&state) {
                continue
            }
            visited.insert(state.clone());
            if self.tiles[state.pos[0]][state.pos[1]] {
                states.push_back(State {
                    pos: state.pos,
                    time: state.time + 1,
                })
            }
            if state.pos[0] != 0 && self.tiles[state.pos[0]-1][state.pos[1]] {
                states.push_back(State {
                    pos: [state.pos[0]-1, state.pos[1]],
                    time: state.time + 1,
                })
            }
            if state.pos[0] != self.tiles.len()-1 && self.tiles[state.pos[0]+1][state.pos[1]] {
                states.push_back(State {
                    pos: [state.pos[0]+1, state.pos[1]],
                    time: state.time + 1,
                })
            }
            if self.tiles[state.pos[0]][state.pos[1]-1] {
                states.push_back(State {
                    pos: [state.pos[0], state.pos[1]-1],
                    time: state.time + 1,
                })
            }
            if self.tiles[state.pos[0]][state.pos[1]+1] {
                states.push_back(State {
                    pos: [state.pos[0], state.pos[1]+1],
                    time: state.time + 1,
                })
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State {
    pos: [usize; 2],
    time: usize,
}

fn run_a(input: &str) -> usize {
    let mut map = Map::parse(input);
    let start = [0, 1];
    let goal = [map.tiles.len()-1, map.tiles[0].len()-2]; 
    map.walk(start, goal);
    map.time-1
}

fn run_b(input: &str) -> usize {
    let mut map = Map::parse(input);
    let start = [0, 1];
    let goal = [map.tiles.len()-1, map.tiles[0].len()-2]; 
    map.walk(start, goal);
    map.walk(goal, start);
    map.walk(start, goal);
    map.time-1
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
mod day24_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 18);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 305);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 54);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 905);
    }
}
