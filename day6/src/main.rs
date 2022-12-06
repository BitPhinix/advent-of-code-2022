use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let chars = input.chars().collect::<Vec<char>>();

    let part_1 = chars
        .windows(4)
        .position(|window| window.iter().collect::<HashSet<&char>>().len() == 4)
        .unwrap()
        + 4;

    println!("Part 1: {}", part_1);

    let part_2 = chars
        .windows(14)
        .position(|window| window.iter().collect::<HashSet<&char>>().len() == 14)
        .unwrap()
        + 14;

    println!("Part 2: {}", part_2);
}
