use std::env;
use aoc22::read_data;
use parseline::parseln;
use std::collections::{VecDeque, HashSet, HashMap, BinaryHeap};

const DAY: &str = "day16";

#[derive(Debug)]
struct Map {
    flow_rates: Vec<usize>,
    distances: Vec<Vec<usize>>,
    start_distances: Vec<usize>,
}

impl Map {
    fn parse(input: &str) -> Self {
        let node_info: HashMap<String, (usize, Vec<String>)> = input.lines()
            .map(|line| {
                let a: Vec<&str> = line.split(' ').collect();
                parseln!(a[4], "rate={};", rate: usize);
                let neighbours = a[8..].iter().map(|s| s.chars().take(2).collect::<String>()).collect();
                (String::from(a[1]), (rate, neighbours))
            }).collect();
        
        let names: Vec<String> = node_info.keys().into_iter().map(String::from).collect();

        // Floyd warshall
        let mut distances: HashMap<(&str, &str), usize> = HashMap::new();

        for i in 0..names.len() {
            for j in 0..names.len() {
                if node_info.get(&names[i]).unwrap().1.contains(&names[j]) {
                    distances.insert((&names[i], &names[j]), 1);
                } else if i == j {
                    distances.insert((&names[i], &names[j]), 0);
                } else {
                    distances.insert((&names[i], &names[j]), 10000);
                }
            }
        }

        for k in 0..names.len() {
            for i in 0..names.len() {
                for j in 0..i {
                    let &a = distances.get(&(&names[i], &names[k])).unwrap();
                    let &b = distances.get(&(&names[k], &names[j])).unwrap();
                    let &c = distances.get(&(&names[i], &names[j])).unwrap();
                    let d = usize::min(a+b, c);

                    distances.entry((&names[i], &names[j])).and_modify(|e| *e = d);
                    distances.entry((&names[j], &names[i])).and_modify(|e| *e = d);
                }
            }
        }

        let filtered_names: Vec<&String> = names.iter().filter(|&name| node_info.get(name).unwrap().0 != 0).collect();


        let flow_rates: Vec<usize> = filtered_names.iter().map(|&name| node_info.get(name).unwrap().0).collect();

        let mut filtered_dists: Vec<Vec<usize>> = vec![vec![0; flow_rates.len()]; flow_rates.len()];
        for i in  0..flow_rates.len() {
            for j in  0..flow_rates.len() {
                filtered_dists[i][j] = *distances.get(&(filtered_names[i], filtered_names[j])).unwrap();
                filtered_dists[j][i] = filtered_dists[i][j];
            }
        }

        let mut start_dists: Vec<usize> = vec![0; flow_rates.len()];
        for i in  0..flow_rates.len() {
            start_dists[i] = *distances.get(&(filtered_names[i], "AA")).unwrap();
        }

        Map {
            flow_rates,
            distances: filtered_dists,
            start_distances: start_dists,
        }
    }
}

fn bfs(map: &Map) -> usize {
    let mut queue = VecDeque::new();
    for (pos, dist) in map.start_distances.iter().enumerate() {
        let state = State {
            score: (30 - 1 - dist) * map.flow_rates[pos],
            time: 30 - 1 - dist,
            open: 1 << pos,
            pos,
        };
        queue.push_back(state);
    }
    
    let mut best = 0;
    while let Some(state) = queue.pop_front() {
        //if state.score + (0..map.distances.len()).filter(|c| ((1<<c) | state.open) == 0).map(|c| map.flow_rates[c]).sum::<usize>() * state.time < best {
        //    continue
        //}
        for (newpos, &dist) in map.distances[state.pos].iter().enumerate() {
            if state.time > dist && state.open & (1 << newpos) == 0 {
                let newtime = state.time - 1 - dist;
                let newstate = State {
                    score: state.score + newtime * map.flow_rates[newpos],
                    time: newtime,
                    pos: newpos,
                    open: state.open | (1 << newpos),
                };
                queue.push_back(newstate);
            } else {
                best = best.max(state.score);
            }
        }
    }
    best
}

/*
fn bfs_heap(map: &Map) -> usize {
    let mut visited = HashSet::new();
    let mut queue = BinaryHeap::new();
    for (pos, dist) in map.start_distances.iter().enumerate() {
        let state = State {
            score: (30 - 1 - dist) * map.flow_rates[pos],
            time: 30 - 1 - dist,
            open: 1 << pos,
            pos,
        };
        visited.insert(state);
        queue.push(state);
    }
    
    let mut best = 0;
    while let Some(state) = queue.pop_front() {
        //if state.score + (0..map.distances.len()).filter(|c| ((1<<c) | state.open) == 0).map(|c| map.flow_rates[c]).sum::<usize>() * state.time < best {
        //    continue
        //}
        for (newpos, &dist) in map.distances[state.pos].iter().enumerate() {
            if state.time > dist && state.open & (1 << newpos) == 0 {
                let newtime = state.time - 1 - dist;
                let newstate = State {
                    score: state.score + newtime * map.flow_rates[newpos],
                    time: newtime,
                    pos: newpos,
                    open: state.open | (1 << newpos),
                };
                visited.insert(newstate);
                queue.push(newstate);
            } else {
                best = best.max(state.score);
            }
        }
    }
    best
}
*/

fn bfs2(map: &Map) -> usize {
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let total_flow: usize = map.flow_rates.iter().sum();
    for (pos1, &dist1) in map.start_distances.iter().enumerate() {
        for (pos2, &dist2) in map.start_distances.iter().enumerate() {
            if pos1 != pos2 {
                let state = if dist1 > dist2 {
                    State2 {
                        score: (26 - 1 - dist1) * map.flow_rates[pos1] + (26 - 1 - dist2) * map.flow_rates[pos2],
                        time: [26 - 1 - dist2, 26 - 1 - dist1],
                        pos: [pos2, pos1],
                        open: (1 << pos1) | (1 << pos2),
                        remaining_flow: total_flow - map.flow_rates[pos1] - map.flow_rates[pos2],
                    }
                } else {
                    State2 {
                        score: (26 - 1 - dist1) * map.flow_rates[pos1] + (26 - 1 - dist2) * map.flow_rates[pos2],
                        time: [26 - 1 - dist1, 26 - 1 - dist2],
                        pos: [pos1, pos2],
                        open: (1 << pos1) | (1 << pos2),
                        remaining_flow: total_flow - map.flow_rates[pos1] - map.flow_rates[pos2],
                    }
                };
                queue.push_back(state);
            }
        }
    }
    
    let mut best = 0;
    while let Some(state) = queue.pop_front() {
        //if state.score + (0..map.distances.len()).filter(|c| ((1<<c) | state.open) == 0).sum::<usize>() * (state.time[0] - 1) < best {
        //    println!("State {:?} has no hope of beating best score, discarding.", state);
        //    continue
        //}
        if visited.contains(&state) {
            //println!("Already visited {:?}", state);
            continue
        }
        visited.insert(state);
        for (newpos, &dist) in map.distances[state.pos[0]].iter().enumerate() {
            if state.time[0] > dist && state.open & (1 << newpos) == 0 {
                let newtime = state.time[0] - 1 - dist;
                let newstate = if newtime < state.time[1] {
                    State2 {
                        score: state.score + newtime * map.flow_rates[newpos],
                        time: [state.time[1], newtime],
                        pos: [state.pos[1], newpos],
                        open: state.open | (1 << newpos),
                        remaining_flow: state.remaining_flow - map.flow_rates[newpos],
                    }
                } else {
                    State2 {
                        score: state.score + newtime * map.flow_rates[newpos],
                        time: [newtime, state.time[1]],
                        pos: [newpos, state.pos[1]],
                        open: state.open | (1 << newpos),
                        remaining_flow: state.remaining_flow - map.flow_rates[newpos],
                    }
                };
                queue.push_back(newstate);
            } else {
                best = best.max(state.score);
            }
        }

        if state.time[1] > 3 {
            let nomove = State2 {
                time: [state.time[1], state.time[1] - 1],
                ..state
            };
            queue.push_back(nomove);
        }
    }
    best
}

#[derive(Hash, Eq, PartialEq, Debug, Copy, Clone)]
struct State {
    score: usize,
    time: usize,
    pos: usize,
    open: usize,
}

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State2 {
    score: usize,
    pos: [usize; 2],
    time: [usize; 2],
    open: usize,
    remaining_flow: usize,
}

fn run_a(input: &str) -> usize {
    let map = Map::parse(input);
    bfs(&map)
}

fn run_b(input: &str) -> usize {
    let map = Map::parse(input);
    bfs2(&map)
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
        assert_eq!(result, 2292);
    }
}
