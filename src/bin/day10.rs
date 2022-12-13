use std::env;
use aoc22::read_data;

const DAY: &str = "day10";

enum Instruction {
    Addx(i64),
    Noop,
}

struct Cpu {
    register: i64,
    cycle: i64,
    instruction: Instruction,
    iterations: i64,
}

impl Cpu {
    fn new() -> Self {
        Cpu { 
            register: 1, 
            cycle: 1, 
            instruction: Instruction::Noop, 
            iterations: 0,
        }
    }

    fn set_instruction(&mut self, line: &str) {
        let cmd: Vec<&str> = line.split(' ').collect();
        if cmd[0] == "noop" {
            self.instruction = Instruction::Noop;
            self.iterations = 1;
        } else {
            self.instruction = Instruction::Addx(cmd[1].parse::<i64>().unwrap());
            self.iterations = 2;
        }
    }

    fn tick(&mut self) -> bool {
        self.cycle += 1;
        self.iterations -= 1;
        if self.iterations == 0 {
            if let Instruction::Addx(x) = self.instruction {
                self.register += x;
            }
            return true
        }
        false
    }
}

fn run_a(input: &str) -> i64 {
    let mut cpu = Cpu::new();
    let mut signal_strength = 0;
    for line in input.lines() {
        cpu.set_instruction(line);
        loop {
            if (cpu.cycle - 20) % 40 == 0 {
                signal_strength += cpu.cycle * cpu.register;
            }
            if cpu.tick() {
                break
            }
        }
    }
    signal_strength
}

fn run_b(input: &str) -> String {
    let mut cpu = Cpu::new();
    let mut screen: String = String::from("");
    for line in input.lines() {
        cpu.set_instruction(line);
        loop {
            println!("{} {} {}", cpu.cycle, cpu.register, cpu.iterations);
            let crt_pos = (cpu.cycle - 1) % 40;
            if cpu.register - 1 <= crt_pos && crt_pos <= cpu.register + 1 {
                screen.push('#');
            } else {
                screen.push('.');
            }
            if cpu.cycle % 40 == 0 {
                screen.push('\n');
            }
            if cpu.tick() {
                break
            }
        }
    }
    String::from(screen.trim_end())
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
        assert_eq!(result, 13140);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 15020);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....");
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, "####.####.#..#..##..#....###...##..###..
#....#....#..#.#..#.#....#..#.#..#.#..#.
###..###..#..#.#....#....#..#.#..#.#..#.
#....#....#..#.#.##.#....###..####.###..
#....#....#..#.#..#.#....#....#..#.#....
####.#.....##...###.####.#....#..#.#....");
    }
}
