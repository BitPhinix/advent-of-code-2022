use std::{collections::HashSet, fs};

#[derive(Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl TryFrom<char> for Direction {
    type Error = ();
    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'U' => Ok(Direction::Up),
            'D' => Ok(Direction::Down),
            'L' => Ok(Direction::Left),
            'R' => Ok(Direction::Right),
            _ => Err(()),
        }
    }
}

struct Rope {
    joints: Vec<(i32, i32)>,
}

impl Rope {
    fn with_size(size: usize) -> Self {
        Self {
            joints: vec![(0, 0); size],
        }
    }
}

impl Rope {
    fn move_head(&mut self, direction: &Direction) {
        let head = self.joints.first_mut().unwrap();
        match direction {
            Direction::Up => head.1 += 1,
            Direction::Down => head.1 -= 1,
            Direction::Left => head.0 -= 1,
            Direction::Right => head.0 += 1,
        }

        let mut previous = self.joints.first().unwrap().clone();
        self.joints.iter_mut().skip(1).for_each(|joint| {
            let delta = ((previous.0 - joint.0), (previous.1 - joint.1));
            if (delta.0.abs() > 1) || (delta.1.abs() > 1) {
                joint.0 += delta.0.signum();
                joint.1 += delta.1.signum();
            }
            previous = joint.clone();
        });
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let moves: Vec<Direction> = input
        .split("\n")
        .flat_map(|s| {
            let (direction, distance) = s.split_once(' ').unwrap();
            let direction = Direction::try_from(direction.chars().nth(0).unwrap()).unwrap();
            let distance = distance.parse::<usize>().unwrap();
            vec![direction; distance]
        })
        .collect();

    let mut rope_1 = Rope::with_size(2);
    let mut visited_1: HashSet<(i32, i32)> = HashSet::new();
    moves.iter().for_each(|direction| {
        rope_1.move_head(direction);
        visited_1.insert(rope_1.joints.last().unwrap().clone());
    });

    println!("Part1: {}", visited_1.len());

    let mut rope_2 = Rope::with_size(10);
    let mut visited_2: HashSet<(i32, i32)> = HashSet::new();
    moves.iter().for_each(|direction| {
        rope_2.move_head(direction);
        visited_2.insert(rope_2.joints.last().unwrap().clone());
    });

    println!("Part2: {}", visited_2.len());
}
