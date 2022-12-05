#[macro_use]
extern crate lazy_static;

use regex::Regex;
use std::fs;

lazy_static! {
    static ref INSTRUCTION_REGEX: Regex =
        Regex::new(r"^move (?P<count>\d+) from (?P<source>\d+) to (?P<destination>\d+)$").unwrap();
}

struct Cargo {
    stacks: Vec<Vec<char>>,
}

impl From<&str> for Cargo {
    fn from(input: &str) -> Self {
        let lines: Vec<&str> = input.lines().collect();
        let (cargo_info, cargo_lines) = lines.split_last().unwrap();
        let stack_count = cargo_info.split_whitespace().count();

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(stack_count);
        for _ in 0..stack_count {
            stacks.push(Vec::new());
        }

        cargo_lines.iter().rev().for_each(|line| {
            let chars: Vec<char> = line.chars().collect();
            for i in 0..stack_count {
                let item = chars[i * 4 + 1];
                if item != ' ' {
                    stacks[i].push(item);
                }
            }
        });

        return Cargo { stacks };
    }
}

impl Cargo {
    fn apply_instruction(mut self: Self, instruction: &str, reverse_picked: bool) -> Self {
        let captures = INSTRUCTION_REGEX.captures(instruction).unwrap();
        let count = captures
            .name("count")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let source = captures
            .name("source")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let destination = captures
            .name("destination")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let source_size = self.stacks[source - 1].len();
        let mut picked = self.stacks[source - 1].split_off(source_size - count);

        if reverse_picked {
            picked.reverse();
        }

        self.stacks[destination - 1].append(&mut picked);
        return self;
    }

    fn solution(self: &Self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap())
            .collect::<String>()
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let (cargo_input, instructions) = input.split_once("\n\n").unwrap();

    let mut cargo_1 = Cargo::from(cargo_input);
    for instruction in instructions.lines() {
        cargo_1 = cargo_1.apply_instruction(instruction, true);
    }

    println!("Part1: {}", cargo_1.solution());

    let mut cargo_2 = Cargo::from(cargo_input);
    for instruction in instructions.lines() {
        cargo_2 = cargo_2.apply_instruction(instruction, false);
    }

    println!("Part2: {}", cargo_2.solution());
}
