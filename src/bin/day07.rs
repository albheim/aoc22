use std::env;
use aoc22::read_data;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

const DAY: &str = "day07";

struct Inode {
    isfile: bool,
    size: u64,
    contents: HashMap<String, Rc<RefCell<Inode>>>,
    parent: Option<Rc<RefCell<Inode>>>
}

impl Inode {
    fn build(input: &str) -> Rc<RefCell<Inode>> {
        let root = Rc::new(RefCell::new(
            Inode {
                parent: None,
                size: 0,
                contents: HashMap::new(),
                isfile: false,
            }));
        let mut currdir = Rc::clone(&root);

        for line in input.lines() {
            let command: Vec<&str> = line.split(' ').collect();
            match (command[0], command[1]) {
                ("$", "cd") => {
                    currdir = match command[2] {
                        ".." => Rc::clone(&currdir.borrow().parent.as_ref().unwrap()),
                        "/" => Rc::clone(&root),
                        x => Rc::clone(currdir.borrow_mut().contents.get(x).unwrap()),
                    };
                }, 
                ("$", "ls") => (),
                ("dir", name) => {
                    currdir.borrow_mut().contents.insert(
                        String::from(name), 
                        Rc::new(RefCell::new(Inode { 
                            isfile: false,
                            size: 0,
                            contents: HashMap::new(),
                            parent: Some(Rc::clone(&currdir)),
                        })));
                    },
                (size, name) => {
                    currdir.borrow_mut().contents.insert(
                        String::from(name), 
                        Rc::new(RefCell::new(Inode { 
                            isfile: true,
                            size: str::parse::<u64>(size).unwrap(),
                            contents: HashMap::new(),
                            parent: Some(Rc::clone(&currdir)),
                        })));
                    },
            }
        }

        root
    }

    fn size(&self) -> (u64, Vec<u64>) {
        let mut folder_size = 0;
        let mut all_folder_sizes = Vec::new();
        for f in self.contents.values() {
            if f.borrow().isfile {
                folder_size += f.borrow().size;
            } else {
                let (fsize, mut all_sizes) = f.borrow().size();
                folder_size += fsize;
                all_folder_sizes.append(&mut all_sizes);
            }
        }
        all_folder_sizes.push(folder_size);
        (folder_size, all_folder_sizes)
    }
}

fn run_a(input: &str) -> u64 {
    let root = Inode::build(input);
    let (_, folder_sizes) = root.borrow().size();
    folder_sizes.iter().filter(|&x| *x <= 100000).sum()
}

fn run_b(input: &str) -> u64 {
    let root = Inode::build(input);
    let (root_size, folder_sizes) = root.borrow().size();
    let unused = 70_000_000 - root_size;
    let to_free = 30_000_000 - unused;

    *folder_sizes.iter().filter(|&x| *x >= to_free).min().unwrap()
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
mod day07_tests {
    use super::*;
    use aoc22::read_test;

    #[test]
    fn test_a() {
        let result = run_a(&read_test(DAY));
        assert_eq!(result, 95437);
    }

    #[test]
    fn test_b() {
        let result = run_b(&read_test(DAY));
        assert_eq!(result, 24933642);
    }

    #[test]
    fn real_a() {
        let result = run_a(&read_data(DAY));
        assert_eq!(result, 1428881);
    }

    #[test]
    fn real_b() {
        let result = run_b(&read_data(DAY));
        assert_eq!(result, 10475598);
    }
}
