use std::env;
use aoc22::read_data;

const DAY: &str = "day18";

fn parse(input: &str) -> Vec<Vec<Vec<bool>>> {
    let mut voxels = vec![vec![vec![false; 22]; 22]; 22];
    for line in input.lines() {
        let a: Vec<usize> = line.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
        voxels[a[0]+1][a[1]+1][a[2]+1] = true;
    }
    voxels
}

fn neighbours((x, y, z): (usize, usize, usize)) -> Vec<(usize, usize, usize)> {
    let min = 0;
    let max = 21;
    let mut v = Vec::new();
    if x < max {
        v.push((x+1, y, z))
    }
    if x > min {
        v.push((x-1, y, z))
    }
    if y < max {
        v.push((x, y+1, z))
    }
    if y > min {
        v.push((x, y-1, z))
    }
    if z < max {
        v.push((x, y, z+1))
    }
    if z > min {
        v.push((x, y, z-1))
    }
    v
}

fn run_a(input: &str) -> usize {
    let voxels = parse(input);
    let mut surface = 0;
    for x in 1..21 {
        for y in 1..21 {
            for z in 1..21 {
                if voxels[x][y][z] {
                    surface += 6;
                    for (xx, yy, zz) in neighbours((x, y, z)) {
                        if voxels[xx][yy][zz] {
                            surface -= 1;
                        }
                    }
                }
            }
        }
    }
    surface
}

fn dfs(voxels: &[Vec<Vec<bool>>], pos: (usize, usize, usize)) -> usize {
    let mut positions = vec![pos];
    let mut visited = vec![vec![vec![false; 22]; 22]; 22];
    let mut tot = 0;

    while let Some((x, y, z)) = positions.pop() {
        visited[x][y][z] = true;

        for (xx, yy, zz) in neighbours((x, y, z)) {
            if voxels[xx][yy][zz] {
                tot += 1;
            } else if !visited[xx][yy][zz] && !positions.contains(&(xx, yy, zz)) {
                positions.push((xx, yy, zz));
            }
        }
    }
    tot
}

fn run_b(input: &str) -> usize {
    let voxels = parse(input);
    dfs(&voxels, (0, 0, 0))
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
mod day18_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 64);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 3466);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 58);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 2012);
    }
}
