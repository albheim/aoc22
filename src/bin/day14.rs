use std::env;
use aoc22::read_data;
use std::collections::HashMap;
use std::str::FromStr;
use std::hash::Hash;
use std::fmt;

const DAY: &str = "day14";

#[derive(Eq, PartialEq, Hash, Clone)]
struct Point {
    x: i64, 
    y: i64,
}

impl FromStr for Point {
    type Err = &'static str;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let a: Vec<&str> = input.split(',').collect();
        if a.len() != 2 {
            return Err("Bad string format.");
        }
        match (a[0].parse::<i64>(), a[1].parse::<i64>()) {
            (Ok(x), Ok(y)) => Ok(Point{ x, y }),
            _ => Err("Failed integer parsing."),
        }
    }
}

enum Material {
    Rock,
    Sand
}

struct Map {
    tiles: HashMap<Point, Material>,
    sand_origin: Point,
    bottom: i64,
}

impl Map {
    fn build(input: &str) -> Map {
        let mut tiles = HashMap::new();
        let mut bottom = i64::MIN;
        for line in input.lines() {
            let points: Vec<Point> = line.split(" -> ")
                .map(|x| x.parse::<Point>().unwrap())
                .collect();
            for i in 1..points.len() {
                let xrange = if points[i-1].x < points[i].x {
                    points[i-1].x..=points[i].x
                } else {
                    points[i].x..=points[i-1].x
                };
                bottom = std::cmp::max(bottom, points[i].y);
                let yrange = if points[i-1].y < points[i].y {
                    points[i-1].y..=points[i].y
                } else {
                    points[i].y..=points[i-1].y
                };
                for x in xrange {
                    for y in yrange.clone() {
                        tiles.insert(Point{ x, y }, Material::Rock);
                    }
                }
            }
        }
        Map {
            tiles,
            bottom,
            sand_origin: Point { x: 500, y: 0 },
        }
    }

    fn drop_sand(&mut self) -> bool {
        let mut sand = self.sand_origin.clone();
        while sand.y < self.bottom {
            sand.y += 1;
            if self.tiles.contains_key(&sand) {
                sand.x -= 1;
                if self.tiles.contains_key(&sand) {
                    sand.x += 2;
                    if self.tiles.contains_key(&sand) {
                        sand.x -= 1;
                        sand.y -= 1;
                        self.tiles.insert(sand, Material::Sand);
                        return true;
                    }
                }
            }
        }
        false
    }

    fn drop_sand_floor(&mut self) -> bool {
        let mut sand = self.sand_origin.clone();
        while sand.y < self.bottom + 1 {
            sand.y += 1;
            if self.tiles.contains_key(&sand) {
                sand.x -= 1;
                if self.tiles.contains_key(&sand) {
                    sand.x += 2;
                    if self.tiles.contains_key(&sand) {
                        sand.x -= 1;
                        sand.y -= 1;
                        if sand == self.sand_origin {
                            return false
                        }
                        break
                    }
                }
            }
        }
        self.tiles.insert(sand, Material::Sand);
        true
    }

}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut xmin = i64::MAX;
        let mut xmax = i64::MIN;
        for pos in self.tiles.keys() {
            if pos.x > xmax {
                xmax = pos.x;
            }
            if pos.x < xmin {
                xmin = pos.x;
            }
        }
        for y in 0..=self.bottom {
            for x in xmin..=xmax {
                let p = Point { x, y };
                if p == self.sand_origin {
                    write!(f, "S")?;
                } else {
                    match self.tiles.get(&p) {
                        Some(Material::Rock) => write!(f, "#")?,
                        Some(Material::Sand) => write!(f, "o")?,
                        None => write!(f, ".")?,
                    }
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn run_a(input: &str) -> usize {
    let mut map = Map::build(input);
    let mut count = 0;
    while map.drop_sand() {
        count += 1;
    }
    count
}

fn run_b(input: &str) -> usize {
    let mut map = Map::build(input);
    let mut count = 0;
    while map.drop_sand_floor() {
        count += 1;
    }
    count + 1
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
        assert_eq!(result, 24);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 817);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 93);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 23111);
    }
}
