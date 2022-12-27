use std::env;
use aoc22::read_data;

const DAY: &str = "day22";

#[derive(Debug, Copy, Clone)]
enum Tile {
    Empty,
    Open,
    Wall,
}

#[derive(Debug)]
enum Command {
    Move(usize),
    Rotate(isize),
}

#[derive(Debug)]
struct State {
    pos: [usize; 2],
    rot: isize
}

#[derive(Debug)]
struct Problem {
    tiles: Vec<Vec<Tile>>,
    commands: Vec<Command>,
    cube_settings: Option<CubeOrientation>,
}

#[derive(Debug, Clone, Copy)]
struct Side {
    face: usize,
    above: usize,
    right: usize,
    pos: [usize; 2],
}

#[derive(Debug)]
struct CubeOrientation {
    sidelen: usize, 
    sides: [Side; 6],
}

impl Problem {
    fn parse(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();

        let mut tiles = Vec::new();
        let mut longest = 0;
        tiles.push(vec![]);
        for &line in lines.iter().take_while(|&&line| !line.is_empty()) {
            let mut row = Vec::new();
            row.push(Tile::Empty);
            for c in line.chars() {
                match c {
                    ' ' => row.push(Tile::Empty),
                    '.' => row.push(Tile::Open),
                    '#' => row.push(Tile::Wall),
                    _ => panic!("Invalid tile input"),
                }
            }
            row.push(Tile::Empty);
            longest = longest.max(row.len());
            tiles.push(row);
        }
        tiles.push(vec![]);

        for t in tiles.iter_mut() {
            while t.len() < longest {
                t.push(Tile::Empty);
            }
        }

        let mut commands = Vec::new();
        let mut tmp = String::new();
        for c in lines.last().unwrap().chars() {
            match c {
                'L' => {
                    commands.push(Command::Move(tmp.parse().unwrap()));
                    commands.push(Command::Rotate(-1));
                    tmp.clear();
                },
                'R' => {
                    commands.push(Command::Move(tmp.parse().unwrap()));
                    commands.push(Command::Rotate(1));
                    tmp.clear();
                },
                '0'..='9' => tmp.push(c),
                _ => panic!("Invalid command input"),
            }
        }
        if !tmp.is_empty() {
            commands.push(Command::Move(tmp.parse().unwrap()));
        }

        Problem { tiles, commands, cube_settings: None }
    }

    fn get_face(right: usize, above: usize) -> usize {
        match (above, right) {
            (5, 4) => 1,
            (4, 2) => 1,
            (2, 3) => 1,
            (3, 5) => 1,

            (1, 4) => 2,
            (4, 6) => 2,
            (6, 3) => 2,
            (3, 1) => 2,

            (5, 1) => 3,
            (1, 2) => 3,
            (2, 6) => 3,
            (6, 5) => 3,

            (1, 5) => 4,
            (5, 6) => 4,
            (6, 2) => 4,
            (2, 1) => 4,

            (6, 4) => 5,
            (4, 1) => 5,
            (1, 3) => 5,
            (3, 6) => 5,

            (2, 4) => 6,
            (4, 5) => 6,
            (5, 3) => 6,
            (3, 2) => 6,

            _ => panic!("invalid face configuration"),
        }
    }

    fn cube_settings(&mut self) {
        let total_tiles: usize = self.tiles.iter().map(|row| row.iter().filter(|c| matches!(c, Tile::Open | Tile::Wall)).count()).sum();
        let sidelen = ((total_tiles / 6) as f64).sqrt() as usize;
        let mut sides = [Side {
            face: 10,
            above: 10,
            right: 10,
            pos: [0, 0],
        }; 6];

        sides[0].face = 1;
        sides[0].above = 5;
        sides[0].right = 4;
        sides[0].pos[0] = 1;
        sides[0].pos[1] = self.tiles[1].iter().position(|t| match *t {
            Tile::Empty => false,
            Tile::Wall | Tile::Open => true,
        }).unwrap();

        let mut visited = vec![1];
        let mut dfs = vec![1];

        while let Some(curr_face) = dfs.pop() {
            let curr_side = sides[curr_face - 1];
            let pos = curr_side.pos;

            if pos[0] > sidelen && !visited.contains(&curr_side.above) && matches!(self.tiles[pos[0] - sidelen][pos[1]], Tile::Wall | Tile::Open) {
                let face = curr_side.above;
                sides[face - 1].face = face;
                sides[face - 1].right = Problem::get_face(face, curr_face);
                sides[face - 1].above = 7 - curr_face;
                sides[face - 1].pos = [pos[0] - sidelen, pos[1]];
                dfs.push(face);
                visited.push(face);
            }
            if pos[1] > sidelen && !visited.contains(&(7 - curr_side.right)) && matches!(self.tiles[pos[0]][pos[1] - sidelen], Tile::Wall | Tile::Open) {
                let face = 7 - curr_side.right;
                sides[face - 1].face = face;
                sides[face - 1].right = curr_face;
                sides[face - 1].above = Problem::get_face(face, curr_face);
                sides[face - 1].pos = [pos[0], pos[1] - sidelen];
                dfs.push(face);
                visited.push(face);
            }
            if pos[0] + sidelen < self.tiles.len() && !visited.contains(&(7 - curr_side.above)) && matches!(self.tiles[pos[0] + sidelen][pos[1]], Tile::Wall | Tile::Open) {
                let face = 7 - curr_side.above;
                sides[face - 1].face = face;
                sides[face - 1].right = Problem::get_face(curr_face, face);
                sides[face - 1].above = curr_face;
                sides[face - 1].pos = [pos[0] + sidelen, pos[1]];
                dfs.push(face);
                visited.push(face);
            }
            if pos[1] + sidelen < self.tiles[0].len() && !visited.contains(&curr_side.right) && matches!(self.tiles[pos[0]][pos[1] + sidelen], Tile::Wall | Tile::Open) {
                let face = curr_side.right;
                sides[face - 1].face = face;
                sides[face - 1].right = 7 - curr_face;
                sides[face - 1].above = Problem::get_face(curr_face, face);
                sides[face - 1].pos = [pos[0], pos[1] + sidelen];
                dfs.push(face);
                visited.push(face);
            }
        }

        self.cube_settings = Some(CubeOrientation { sidelen, sides })
    }

    fn get_side(&self, pos: [usize; 2]) -> Side {
        let cs = self.cube_settings.as_ref().unwrap();
        let sidelen = cs.sidelen;
        for i in 0..6 {
            let side = cs.sides[i];
            if side.pos[0] <= pos[0] && side.pos[0] + sidelen > pos[0] && side.pos[1] <= pos[1] && side.pos[1] + sidelen > pos[1] {
                return cs.sides[i];
            }
        }
        panic!("Fail");
    }

    fn find_edge_jump(&self, state: &State) -> State {
        // Use cube orientation to find square and side, calculate which square and orientation
        let sidelen = self.cube_settings.as_ref().unwrap().sidelen;

        let side: Side = Problem::get_side(self, state.pos);

        let target_face = match state.rot {
            0 => side.right,
            1 => 7 - side.above,
            2 => 7 - side.right,
            3 => side.above,
            _ => panic!("nope"),
        };
        let target_side = self.cube_settings.as_ref().unwrap().sides[target_face - 1];

        let steps = match state.rot {
            0 => state.pos[0] - side.pos[0],
            1 => side.pos[1] + sidelen - 1 - state.pos[1],
            2 => side.pos[0] + sidelen - 1 - state.pos[0],
            3 => state.pos[1] - side.pos[1],
            _ => panic!("nope"),
        };


        if target_side.above == side.face {
            State {
                pos: [target_side.pos[0], target_side.pos[1] + sidelen - 1 - steps],
                rot: 1,
            }
        } else if target_side.right == side.face {
            State {
                pos: [target_side.pos[0] + sidelen - 1 - steps, target_side.pos[1] + sidelen - 1],
                rot: 2,
            }
        } else if 7 - target_side.above == side.face {
            State {
                pos: [target_side.pos[0] + sidelen - 1, target_side.pos[1] + steps],
                rot: 3,
            }
        } else { //if 7 - target_side.right == side.face {
            State {
                pos: [target_side.pos[0] + steps, target_side.pos[1]],
                rot: 0,
            }
        }
    }

    fn step_cube(&self, state: &mut State, dist: usize) {
        for _ in 0..dist {
            let newpos = match state.rot {
                0 => [state.pos[0], state.pos[1]+1],
                1 => [state.pos[0]+1, state.pos[1]],
                2 => [state.pos[0], state.pos[1]-1],
                3 => [state.pos[0]-1, state.pos[1]],
                _ => panic!("Invalid rotation")
            };
            match self.tiles[newpos[0]][newpos[1]] {
                Tile::Open => state.pos = newpos,
                Tile::Wall => break,
                Tile::Empty => {
                    let newstate = Self::find_edge_jump(self, state);
                    if let Tile::Open = self.tiles[newstate.pos[0]][newstate.pos[1]] {
                        *state = newstate;
                    } else {
                        break;
                    }
                },
            }
        }
    }

    fn step_flat(&self, state: &mut State, dist: usize) {
        for _ in 0..dist {
            let newpos = match state.rot {
                0 => [state.pos[0], state.pos[1]+1],
                1 => [state.pos[0]+1, state.pos[1]],
                2 => [state.pos[0], state.pos[1]-1],
                3 => [state.pos[0]-1, state.pos[1]],
                _ => panic!("Invalid rotation")
            };
            match self.tiles[newpos[0]][newpos[1]] {
                Tile::Open => state.pos = newpos,
                Tile::Wall => break,
                Tile::Empty => {
                    let mut newpos = match state.rot {
                        0 => [state.pos[0], 0],
                        1 => [0, state.pos[1]],
                        2 => [state.pos[0], self.tiles[0].len()-1],
                        3 => [self.tiles.len()-1, state.pos[1]],
                        _ => panic!("Invalid rotation"),
                    };
                    while let Tile::Empty = self.tiles[newpos[0]][newpos[1]] {
                        match state.rot {
                            0 => newpos[1] += 1,
                            1 => newpos[0] += 1,
                            2 => newpos[1] -= 1,
                            3 => newpos[0] -= 1,
                            _ => panic!("Invalid rotation"),
                        };
                    }
                    if let Tile::Open = self.tiles[newpos[0]][newpos[1]] {
                        state.pos = newpos;
                    } else {
                        break;
                    }
                },
            }
        }
    }
}

fn run_a(input: &str) -> usize {
    let prob = Problem::parse(input);

    let startx = prob.tiles[1].iter()
        .position(|e| matches!(*e, Tile::Open))
        .unwrap();

    let mut state = State {
        pos: [1, startx],
        rot: 0,
    };

    for command in prob.commands.iter() {
        match *command {
            Command::Move(d) => prob.step_flat(&mut state, d),
            Command::Rotate(d) => state.rot = (state.rot + d).rem_euclid(4),
        }
    }
    1000 * state.pos[0] + 4 * state.pos[1] + state.rot as usize
}

fn run_b(input: &str) -> usize {
    let mut prob = Problem::parse(input);

    prob.cube_settings();

    let startx = prob.tiles[1].iter()
        .position(|e| matches!(*e, Tile::Open))
        .unwrap();

    let mut state = State {
        pos: [1, startx],
        rot: 0,
    };

    for command in prob.commands.iter() {
        match *command {
            Command::Move(d) => prob.step_cube(&mut state, d),
            Command::Rotate(d) => state.rot = (state.rot + d).rem_euclid(4),
        }
    }
    1000 * state.pos[0] + 4 * state.pos[1] + state.rot as usize
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
mod day22_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 6032);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 75254);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 5031);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 108311);
    }
}
