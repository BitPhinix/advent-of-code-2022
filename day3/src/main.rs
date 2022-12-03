use std::{collections::HashSet, fs};

#[derive(Clone, PartialEq, Eq, Hash)]
struct Item {
    priority: u32,
}

impl TryFrom<char> for Item {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let char_code = value as u32;
        match char_code {
            97..=122 => Ok(Self {
                priority: char_code - 96,
            }),
            65..=90 => Ok(Self {
                priority: char_code - 65 + 27,
            }),
            _ => Err(()),
        }
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let backpacks: Vec<Vec<Item>> = input
        .lines()
        .map(|line| line.chars().map(|c| Item::try_from(c).unwrap()).collect())
        .collect();

    let part_1: u32 = backpacks
        .iter()
        .map(|backpack| {
            let (compartment_a, compartment_b) = backpack.split_at(backpack.len() / 2);
            let set_a: HashSet<&Item> = HashSet::from_iter(compartment_a.iter());

            compartment_b
                .iter()
                .find(|item| set_a.contains(item))
                .unwrap()
                .priority
        })
        .sum();

    println!("Part1: {}", part_1);

    let part_2: u32 = backpacks
        .chunks(3)
        .map(|members| {
            let unique_items: Vec<HashSet<Item>> = members
                .iter()
                .map(|backpack| HashSet::from_iter(backpack.iter().cloned()))
                .collect();

            unique_items[0]
                .iter()
                .find(|item| unique_items[1..].iter().all(|set| set.contains(item)))
                .unwrap()
                .priority
        })
        .sum();

    println!("Part2: {}", part_2);
}
