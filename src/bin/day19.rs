use std::env;
use aoc22::read_data;
use parseline::parseln;
use std::collections::{HashMap, HashSet};

const DAY: &str = "day19";

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct State {
    bots: [usize; 4],
    resources: [usize; 4],
    time: usize,
}

#[derive(Debug)]
struct Blueprint {
    id: usize,
    orebot_cost: usize,
    claybot_cost: usize,
    obsidianbot_cost: [usize; 2],
    geodebot_cost: [usize; 2],
}

impl Blueprint {
    fn parse(line: &str) -> Self {
        parseln!(line, "Blueprint {}: Each ore robot costs {} ore. Each clay robot costs {} ore. Each obsidian robot costs {} ore and {} clay. Each geode robot costs {} ore and {} obsidian.", id: usize, ore_ore: usize, clay_ore: usize, obsidian_ore: usize, obsidian_clay: usize, geode_ore: usize, geode_obsidian: usize);
        Blueprint {
            id,
            orebot_cost: ore_ore,
            claybot_cost: clay_ore,
            obsidianbot_cost: [obsidian_ore, obsidian_clay],
            geodebot_cost: [geode_ore, geode_obsidian],
        }
    }

    fn dfs(&self, state: State, visited: &mut HashMap<State, usize>, best_score: usize) -> usize {
        //println!("{:?}", state);

        if state.time == 0 {
            //println!("Best score {} current score {}", best_score, state.resources[3]);
            return state.resources[3];
        }
        if let Some(x) = visited.get(&state) {
            //println!("hello");
            return *x;
        }
        let max_score = state.resources[3] + state.bots[3] * state.time + state.time * (state.time + 1) / 2;
        if max_score < best_score {
            //println!("Max possible from {:?} is {}, best is {}", state, max_score, best_score);
            return 0;
        }

        // Heuristic, max possible score = current geodes + current geode bots * time + time * (time + 1) / 2

        let mut best = best_score;
        let mut cant_produce = 2 + (state.bots[1] > 0) as usize + (state.bots[2] > 0) as usize;

        if state.resources[0] >= self.orebot_cost {
            best = self.dfs(State {
                bots: [state.bots[0]+1, state.bots[1], state.bots[2], state.bots[3]], 
                resources: [state.resources[0]-self.orebot_cost+state.bots[0], state.resources[1]+state.bots[1], state.resources[2]+state.bots[2], state.resources[3]+state.bots[3]], 
                time: state.time - 1,
            }, visited, best).max(best);
            cant_produce -= 1;
        }
        if state.resources[0] >= self.claybot_cost {
            best = self.dfs(State {
                bots: [state.bots[0], state.bots[1]+1, state.bots[2], state.bots[3]], 
                resources: [state.resources[0]-self.claybot_cost+state.bots[0], state.resources[1]+state.bots[1], state.resources[2]+state.bots[2], state.resources[3]+state.bots[3]], 
                time: state.time - 1,
            }, visited, best).max(best);
            cant_produce -= 1;
        }
        if state.resources[0] >= self.obsidianbot_cost[0] && state.resources[1] >= self.obsidianbot_cost[1] {
            best = self.dfs(State {
                bots: [state.bots[0], state.bots[1], state.bots[2]+1, state.bots[3]], 
                resources: [state.resources[0]-self.obsidianbot_cost[0]+state.bots[0], state.resources[1]-self.obsidianbot_cost[1]+state.bots[1], state.resources[2]+state.bots[2], state.resources[3]+state.bots[3]], 
                time: state.time - 1,
            }, visited, best).max(best);
            cant_produce -= 1;
        }
        if state.resources[0] >= self.geodebot_cost[0] && state.resources[2] >= self.geodebot_cost[1] {
            best = self.dfs(State {
                bots: [state.bots[0], state.bots[1], state.bots[2], state.bots[3]+1], 
                resources: [state.resources[0]-self.geodebot_cost[0]+state.bots[0], state.resources[1]+state.bots[1], state.resources[2]-self.geodebot_cost[1]+state.bots[2], state.resources[3]+state.bots[3]], 
                time: state.time - 1,
            }, visited, best).max(best);
            cant_produce -= 1;
        }
        if cant_produce > 0 {
            best = self.dfs(State {
                bots: [state.bots[0], state.bots[1], state.bots[2], state.bots[3]], 
                resources: [state.resources[0]+state.bots[0], state.resources[1]+state.bots[1], state.resources[2]+state.bots[2], state.resources[3]+state.bots[3]], 
                time: state.time - 1,
            }, visited, best).max(best);
        }

        let e = visited.entry(state).or_insert(0);
        if *e < best {
            *e = best;
        }
        best
    }
}



fn run_a(input: &str) -> usize {
    input.lines()
        .map(Blueprint::parse)
        .map(|bp| {
            let val = bp.dfs(State {
                bots: [1, 0, 0, 0],
                resources: [0; 4],
                time: 24,
            }, &mut HashMap::new(), 0);
            println!("{} {}", bp.id, val);
            val * bp.id
        }).sum()
}

fn run_b(input: &str) -> usize {
    input.lines()
        .take(3)
        .map(Blueprint::parse)
        .map(|bp| {
            let val = bp.dfs(State {
                bots: [1, 0, 0, 0],
                resources: [0; 4],
                time: 32,
            }, &mut HashMap::new(), 0);
            println!("{} {}", bp.id, val);
            val
        }).product()
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
mod day19_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 33);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 1023);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 56 * 62);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 13520);
    }
}
