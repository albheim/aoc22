use std::env;
use aoc22::read_data;
use parseline::parseln;
use std::collections::{HashSet, HashMap};

const DAY: &str = "day16";

#[derive(Debug)]
struct Node { // Can probably make this faster by making String into usize
    id: usize,
    flow_rate: usize,
    neighbours: HashSet<usize>,
}

fn to_id(s: &str) -> usize {
    let mut a = 0;
    for c in s.bytes() {
        a <<= 8;
        a |= c as usize;
    }
    a
}

impl Node {
    fn parse(line: &str) -> Self {
        parseln!(line, "Valve {} has flow rate={}; tunnel{} lead{} to valve{} {}", id: String, flow_rate: usize, _tmp: String, _tmp2: String, _tmp3: String, other: String);
        let neighbours = other.split(", ").map(to_id).collect();
        let n = Node {
            id: to_id(&id),
            flow_rate,
            neighbours,
        };
        println!("Parsed {:?}", n);
        n
    }
}

struct Map {
    nodes: HashMap<usize, Node>,
    distances: HashMap<(usize, usize), usize>,
}

impl Map {
    fn find_dist(nodes: &HashMap<usize, Node>, origin: usize, curr: usize, dist: usize, distances: &mut HashMap<(usize, usize), usize>) {
        if let Some(&a) = distances.get(&(origin, curr)) {
            if dist >= a {
                return 
            }
        } 
        distances.insert((origin, curr), dist);
        for &n in nodes.get(&curr).unwrap().neighbours.iter() {
            Map::find_dist(nodes, origin, n, dist + 1, distances);
        }
    }

    fn parse(input: &str) -> Self {
        let nodes: HashMap<_, _> = input.lines().map(|line| {
            let node = Node::parse(line);
            (node.id, node)
        }).collect();
        let mut distances = HashMap::new();
        for &n in nodes.keys() {
            Map::find_dist(&nodes, n, n, 0, &mut distances);
        }
        Map{ nodes, distances }
    }

    fn dfs(map: &Map, state: State, path: &mut Vec<usize>) -> usize {
        //println!("dfs {:?}", state);
        let mut best = state.score;
        for &n in map.nodes.keys() {
            if !state.open.contains(&n) && map.nodes.get(&n).unwrap().flow_rate > 0 {
                if let Some(dur) = map.distances.get(&(state.pos, n)) {
                    let dur = dur + 1;
                    if state.time > dur {
                        let mut open = state.open.clone();
                        open.insert(n);
                        path.push(n);
                        best = Map::dfs(map, State{ 
                            score: state.score + (state.time - dur) * map.nodes.get(&n).unwrap().flow_rate,
                            time: state.time - dur,
                            pos: n,
                            open,
                        }, path).max(best);
                        path.pop();
                    }
                }
            }
        }
        //println!("score {} with time {} left and path {:?}", best, state.time, path);
        best
    }

    fn dfs2(map: &Map, state: State2, path: &mut Vec<usize>) -> usize {
        println!("dfs {:?}", state);
        let mut best = state.score;
        for &n in map.nodes.keys() {
            if !state.open.contains(&n) && map.nodes.get(&n).unwrap().flow_rate > 0 {
                if state.time.0 < state.time.1 {
                    if let Some(dur) = map.distances.get(&(state.pos.1, n)) {
                        let dur = dur + 1;
                        if state.time.1 > dur {
                            let mut open = state.open.clone();
                            open.insert(n);
                            path.push(n);
                            best = Map::dfs2(map, State2 { 
                                score: state.score + (state.time.1 - dur) * map.nodes.get(&n).unwrap().flow_rate,
                                time: (state.time.0, state.time.1 - dur),
                                pos: (state.pos.0, n),
                                open,
                            }, path).max(best);
                            path.pop();
                        }
                    }
                } else {
                    if let Some(dur) = map.distances.get(&(state.pos.0, n)) {
                        let dur = dur + 1;
                        if state.time.0 > dur {
                            let mut open = state.open.clone();
                            open.insert(n);
                            path.push(n);
                            best = Map::dfs2(map, State2 { 
                                score: state.score + (state.time.0 - dur) * map.nodes.get(&n).unwrap().flow_rate,
                                time: (state.time.0 - dur, state.time.1),
                                pos: (n, state.pos.1),
                                open,
                            }, path).max(best);
                            path.pop();
                        }
                    }
                }
            }
        }
        //println!("score {} with time {} left and path {:?}", best, state.time, path);
        best
    }
}

#[derive(Debug)]
struct State {
    score: usize,
    time: usize,
    pos: usize,
    open: HashSet<usize>,
}

#[derive(Debug)]
struct State2 {
    score: usize,
    time: (usize, usize),
    pos: (usize, usize),
    open: HashSet<usize>,
}

fn run_a(input: &str) -> usize {
    let map = Map::parse(input);

    let state = State {
        score: 0,
        time: 30,
        pos: to_id("AA"),
        open: HashSet::new(),
    };

    let mut v = vec![];
    Map::dfs(&map, state, &mut v)
}

fn run_b(input: &str) -> usize {
    let map = Map::parse(input);

    let state = State2 {
        score: 0,
        time: (26, 26),
        pos: (to_id("AA"), to_id("AA")),
        open: HashSet::new(),
    };

    let mut v = vec![];
    Map::dfs2(&map, state, &mut v)
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
mod day16_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 1651);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 1828);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 1707);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 23416);
    }
}
