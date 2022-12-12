use std::env;
use aoc22::read_data;
use queues::*;

const DAY: &str = "day12";

struct Map {
    tiles: Vec<Vec<isize>>,
    start: (usize, usize),
    goal: (usize, usize),
}

impl Map {
    fn build(input: &str) -> Map {
        let mut start = (0, 0);
        let mut goal = (0, 0);
        let mut x = 0;
        let mut y = 0;
        let tiles: Vec<Vec<isize>> = input.lines()
            .map(|line| {
                y += 1;
                x = 0;
                line.chars().map(|c| {
                    x += 1;
                    if c == 'S' {
                        start = (x-1, y-1);
                        0
                    } else if c == 'E' {
                        goal = (x-1, y-1);
                        25
                    } else {
                        c as isize - 'a' as isize
                    }
                }).collect()
            }).collect();
        
        Map { tiles, start, goal }
    }

    fn get_adjacent(&self, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
        let max_height = self.tiles[y][x] - 1;
        let mut adjacent = Vec::new();
        if x > 0 && self.tiles[y][x - 1] >= max_height {
            adjacent.push((x - 1, y))
        }
        if x + 1 < self.tiles[0].len() && self.tiles[y][x + 1] >= max_height {
            adjacent.push((x + 1, y))
        }
        if y > 0 && self.tiles[y - 1][x] >= max_height {
            adjacent.push((x, y - 1))
        }
        if y + 1 < self.tiles.len() && self.tiles[y + 1][x] >= max_height {
            adjacent.push((x, y + 1))
        }
        adjacent
    }
}

fn shortest_path(map: &Map) -> Vec<Vec<usize>> {
    let mut floodfill = vec![vec![usize::MAX; map.tiles[0].len()]; map.tiles.len()];
    floodfill[map.goal.1][map.goal.0] = 0;
    let mut queue = queue![map.goal];
    while queue.size() > 0 {
        let (x, y) = queue.remove().unwrap();
        for (xn, yn) in map.get_adjacent((x, y)) {
            if floodfill[yn][xn] == usize::MAX {
                queue.add((xn, yn)).unwrap();
                floodfill[yn][xn] = floodfill[y][x] + 1;
            } else if floodfill[yn][xn] > floodfill[y][x] + 1 {
                floodfill[yn][xn] = floodfill[y][x] + 1;
            }
        }
    }
    floodfill
}

fn run_a(input: &str) -> usize {
    let map = Map::build(input);
    let floodfill = shortest_path(&map);
    floodfill[map.start.1][map.start.0]
}

fn run_b(input: &str) -> usize {
    let map = Map::build(input);
    let floodfill = shortest_path(&map);
    let mut min = floodfill.len() * floodfill[0].len();
    for y in 0..floodfill.len() {
        for x in 0..floodfill[0].len() {
            if map.tiles[y][x] == 0 && floodfill[y][x] < min {
                min = floodfill[y][x];
            }
        }
    }
    min
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
        assert_eq!(result, 31);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 490);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 29);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 488);
    }
}
