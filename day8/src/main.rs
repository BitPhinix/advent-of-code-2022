use std::{collections::HashSet, fs};

struct Forest {
    trees: Vec<Vec<u32>>,
}

impl Forest {
    fn columns(&self) -> usize {
        if self.trees.is_empty() || self.trees.first().unwrap().is_empty() {
            return 0;
        }

        self.trees.first().unwrap().len()
    }

    fn rows(&self) -> usize {
        self.trees.len()
    }

    fn iter(&self, position: (usize, usize), step: (i32, i32)) -> ForestIterator {
        ForestIterator {
            forest: self,
            current: (position.0 as i32, position.1 as i32),
            step,
        }
    }

    fn iter_col(&self, col: usize, reverse: bool) -> ForestIterator {
        ForestIterator::new_col(self, col, reverse).unwrap()
    }

    fn iter_row(&self, row: usize, reverse: bool) -> ForestIterator {
        ForestIterator::new_row(self, row, reverse).unwrap()
    }

    fn scenic_score(&self, position: (usize, usize)) -> Option<usize> {
        let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
        Some(
            directions
                .iter()
                .map(|direction| {
                    return self
                        .iter(position, *direction)
                        .viewing_distance()
                        .unwrap_or(0);
                })
                .product(),
        )
    }
}

impl TryFrom<&String> for Forest {
    type Error = ();

    fn try_from(value: &String) -> Result<Self, Self::Error> {
        let trees: Vec<Vec<u32>> = value
            .lines()
            .map(|line| {
                line.chars()
                    .map(|char| char.to_digit(10).ok_or(()))
                    .collect()
            })
            .collect::<Result<Vec<Vec<u32>>, ()>>()?;

        Ok(Self { trees })
    }
}

struct ForestIterator<'a> {
    forest: &'a Forest,
    current: (i32, i32),
    step: (i32, i32),
}

impl<'a> ForestIterator<'a> {
    fn new_row(forest: &'a Forest, row: usize, reverse: bool) -> Option<Self> {
        let current: (i32, i32) = match reverse {
            true => (row as i32, forest.columns() as i32 - 1),
            false => (row as i32, 0),
        };

        let step = match reverse {
            true => (0, -1),
            false => (0, 1),
        };

        Some(Self {
            forest,
            current,
            step,
        })
    }

    fn new_col(forest: &'a Forest, col: usize, reverse: bool) -> Option<Self> {
        let current = match reverse {
            true => (forest.rows() as i32 - 1, col as i32),
            false => (0, col as i32),
        };

        let step = match reverse {
            true => (-1, 0),
            false => (1, 0),
        };

        Some(Self {
            forest,
            current,
            step,
        })
    }

    fn visible(&mut self, current: Option<u32>) -> Vec<(u32, (usize, usize))> {
        let mut max: Option<u32> = current;
        self.into_iter()
            .filter(|(value, _)| {
                let is_visible = match max {
                    Some(max) => *value > max,
                    _ => true,
                };

                if is_visible {
                    max = Some(*value);
                    return true;
                }

                false
            })
            .collect()
    }

    fn viewing_distance(&mut self) -> Option<usize> {
        let view_line: Vec<_> = self.into_iter().collect();
        let current = view_line.first()?.0;

        Some(
            view_line
                .iter()
                .skip(1)
                .position(|(value, _)| value >= &current)
                .map(|pos| pos + 1)
                .unwrap_or(view_line.len() - 1),
        )
    }
}

impl Iterator for ForestIterator<'_> {
    type Item = (u32, (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.current;
        let current_row = &self.forest.trees.get(row as usize)?;
        let value = current_row.get(col as usize)?;

        let (row_step, col_step) = self.step;
        self.current = (row + row_step, col + col_step);
        Some((value.to_owned(), (row as usize, col as usize)))
    }
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();
    let forest = Forest::try_from(&input).unwrap();

    let directions = [true, false];
    let mut visible: HashSet<(usize, usize)> = HashSet::new();
    for row in 0..forest.rows() {
        for &reverse in directions.iter() {
            forest
                .iter_row(row, reverse)
                .visible(None)
                .iter()
                .for_each(|(_, pos)| {
                    visible.insert(pos.to_owned());
                });
        }
    }
    for col in 0..forest.columns() {
        for &reverse in directions.iter() {
            forest
                .iter_col(col, reverse)
                .visible(None)
                .iter()
                .for_each(|(_, pos)| {
                    visible.insert(pos.to_owned());
                });
        }
    }

    println!("Part 1: {}", visible.len());

    let mut max_scenic_score = 0;
    for row in 0..forest.rows() {
        for col in 0..forest.columns() {
            let score = forest.scenic_score((row, col)).unwrap();
            if score > max_scenic_score {
                max_scenic_score = score;
            }
        }
    }

    println!("Part 2: {}", max_scenic_score);
}
