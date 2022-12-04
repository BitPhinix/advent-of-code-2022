use std::fs;

#[derive(Clone, PartialEq, Eq, Hash)]
struct Range {
    start: u32,
    end: u32,
}

impl TryFrom<&str> for Range {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value.split_once('-') {
            None => Err(()),
            Some((start, end)) => Ok(Range {
                start: start.parse().map_err(|_| ())?,
                end: end.parse().map_err(|_| ())?,
            }),
        }
    }
}

impl Range {
    fn contains(&self, other: &Range) -> bool {
        self.start >= other.start && self.end <= other.end
    }

    fn intersects(&self, other: &Range) -> bool {
        self.start <= other.end && self.end >= other.start
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let ranges: Vec<(Range, Range)> = input
        .lines()
        .map(|line| {
            let (a, b) = line.split_once(',').unwrap();
            (a.try_into().unwrap(), b.try_into().unwrap())
        })
        .collect();

    let part_1 = ranges
        .iter()
        .filter(|(a, b)| a.contains(b) || b.contains(a))
        .count();

    println!("Part1: {}", part_1);

    let part_2 = ranges.iter().filter(|(a, b)| a.intersects(b)).count();

    println!("Part2: {}", part_2);
}
