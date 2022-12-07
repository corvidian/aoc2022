#![feature(is_some_and)]

use std::collections::BTreeMap;

struct Dir {
    children: BTreeMap<String, Dir>,
    size: u32,
}

const MAX_SIZE: u32 = 100000;
const TOTAL_SPACE: u32 = 70000000;
const NEEDED_SPACE: u32 = 30000000;

impl Dir {
    fn new() -> Dir {
        Dir {
            children: BTreeMap::new(),
            size: 0,
        }
    }

    fn get_child<'a>(&'a mut self, path: &[String]) -> &'a mut Dir {
        path.iter()
            .fold(self, |dir, child| dir.children.get_mut(child).unwrap())
    }

    fn size(&self) -> u32 {
        self.children
            .values()
            .map(|child| child.size())
            .sum::<u32>()
            + self.size
    }

    fn visualize(&self) {
        self.visualize_child("/", 0);
    }

    fn visualize_child(&self, name: &str, indent: usize) {
        let spacer = " ".repeat(indent);
        println!("{spacer}{name} ({})", self.size());
        self.children
            .iter()
            .for_each(|(name, child)| child.visualize_child(name, indent + 2));
    }

    fn sizes(&self) -> Vec<u32> {
        let mut flat = self
            .children
            .values()
            .flat_map(|child| child.sizes())
            .collect::<Vec<_>>();
        flat.push(self.size());
        flat
    }
}

fn main() {
    let mut root: Dir = Dir::new();
    let mut current_path: Vec<String> = vec![];
    let mut lines = aoc::read_input_lines();
    lines.reverse();
    while let Some(command) = lines.pop() {
        if command.starts_with("$ ls") {
            while lines.last().is_some_and(|s| !s.starts_with('$')) {
                let content = lines.pop().unwrap();
                if content.starts_with("dir") {
                    let name = content.split(' ').last().unwrap().to_string();
                    root.get_child(&current_path)
                        .children
                        .insert(name, Dir::new());
                } else {
                    let size: u32 = content.split(' ').next().unwrap().parse().unwrap();
                    root.get_child(&current_path).size += size;
                }
            }
        } else if command == "$ cd .." {
            current_path.pop();
        } else if command.starts_with("$ cd") {
            let name = command.split(' ').last().unwrap();
            current_path.push(name.to_string());
        }
    }
    root.visualize();
    let mut flat_sizes = root.sizes();
    flat_sizes.sort();

    let part1: u32 = root.sizes().iter().filter(|&size| *size < MAX_SIZE).sum();
    println!("Part 1: {part1}");

    let free_size = TOTAL_SPACE - root.size();
    let part2: &u32 = flat_sizes
        .iter()
        .find(|size| free_size + *size > NEEDED_SPACE)
        .unwrap();
    println!("Part 2: {part2}");
}
