use std::{collections::HashSet, fs};

enum Instruction {
    Noop(),
    AddX(i32),
}

impl TryFrom<&str> for Instruction {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value == "noop" {
            return Ok(Instruction::Noop());
        }

        let (command, param) = value.split_once(' ').ok_or(())?;
        match command {
            "addx" => Ok(Instruction::AddX(param.parse().map_err(|_| ())?)),
            _ => Err(()),
        }
    }
}

struct Cpu {
    register_value: i32,
    cycle: usize,
}

impl Cpu {
    fn new() -> Self {
        Self {
            register_value: 1,
            cycle: 1,
        }
    }

    fn execute(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::AddX(value) => {
                self.register_value += value;
                self.cycle += 2;
            }
            Instruction::Noop() => {
                self.cycle += 1;
            }
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let instructions: Vec<Instruction> = input
        .lines()
        .map(|line| Instruction::try_from(line).unwrap())
        .collect();

    let checkpoints: HashSet<usize> = HashSet::from([20, 60, 100, 140, 180, 220]);

    let mut cpu_1 = Cpu::new();
    let mut part_1 = 0;
    instructions.iter().for_each(|instruction| {
        let cycle_before = cpu_1.cycle;
        let register_before = cpu_1.register_value;
        cpu_1.execute(instruction);

        for cycle in (cycle_before)..(cpu_1.cycle) {
            if checkpoints.contains(&cycle) {
                part_1 += register_before * (cycle as i32);
            }
        }
    });

    println!("Part1: {}", part_1);

    let mut cpu_2 = Cpu::new();
    let mut part_2 = String::new();
    instructions.iter().for_each(|instruction| {
        let cycle_before = cpu_2.cycle;
        let register_before = cpu_2.register_value;
        cpu_2.execute(instruction);

        for cycle in (cycle_before)..(cpu_2.cycle) {
            let position = ((cycle - 1) % 40) as i32;
            let visible = (position - register_before).abs() <= 1;

            if position == 0 {
                part_2.push_str("\n")
            }

            if visible {
                part_2.push_str("#");
            } else {
                part_2.push_str(" ");
            }
        }
    });

    print!("Part2: {}", part_2);
}
