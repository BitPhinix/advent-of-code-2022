use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let mut elves: Vec<i32> = input
        .split("\n\n")
        .map(|items| {
            items
                .split("\n")
                .map(|item| item.parse::<i32>().unwrap())
                .sum()
        })
        .collect();

    elves.sort();

    let max = elves.last().unwrap();
    println!("Part 1: {:?}", max);

    let top_three_sum: i32 = elves.iter().rev().take(3).sum();
    println!("Part 2: {:?}", top_three_sum);
}
