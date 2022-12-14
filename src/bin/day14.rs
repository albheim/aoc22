use std::env;
use aoc22::read_data;
use std::collections::HashMap;
use std::str::FromStr;
use std::hash::Hash;

const DAY: &str = "day14";

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
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

#[derive(Debug)]
enum Material {
    Rock,
    Sand
}

#[derive(Debug)]
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
                let yrange = if points[i-1].y < points[i].y {
                    if bottom < points[i].y {
                        bottom = points[i].y;
                    }
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
                        println!("sand hit {:?}", sand);
                        self.tiles.insert(sand, Material::Sand);
                        return true;
                    }
                }
            }
        }
        false
    }

    fn print(&self) {
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
                    print!("S");
                } else {
                    match self.tiles.get(&p) {
                        Some(Material::Rock) => print!("#"),
                        Some(Material::Sand) => print!("o"),
                        None => print!("."),
                    }
                }
            }
            println!();
        }
    }
}

fn run_a(input: &str) -> usize {
    let mut map = Map::build(input);
    let mut count = 0;
    while map.drop_sand() {
        count += 1;
        println!("count is now {count}");
        map.print();
    }
    count
}

fn run_b(input: &str) -> usize {
    0
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
        assert_eq!(result, 5625);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        //assert_eq!(result, 140);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        //assert_eq!(result, 23111);
    }
}
