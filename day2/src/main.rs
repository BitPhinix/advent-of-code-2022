use std::{fs, str::FromStr};

enum MatchAdvice {
    Win,
    Loose,
    Draw,
}

impl FromStr for MatchAdvice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(MatchAdvice::Loose),
            "Y" => Ok(MatchAdvice::Draw),
            "Z" => Ok(MatchAdvice::Win),
            _ => Err(()),
        }
    }
}

#[derive(PartialEq, Clone)]
enum Pick {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Pick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Pick::Rock),
            "B" => Ok(Pick::Paper),
            "C" => Ok(Pick::Scissors),

            "X" => Ok(Pick::Rock),
            "Y" => Ok(Pick::Paper),
            "Z" => Ok(Pick::Scissors),

            _ => Err(()),
        }
    }
}

impl Pick {
    fn worth(&self) -> i32 {
        match self {
            Pick::Rock => 1,
            Pick::Paper => 2,
            Pick::Scissors => 3,
        }
    }

    fn score_against(&self, other: &Pick) -> i32 {
        match (self, other) {
            (Pick::Rock, Pick::Scissors) => 6,
            (Pick::Paper, Pick::Rock) => 6,
            (Pick::Scissors, Pick::Paper) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        }
    }

    fn pick_against_for_advice(&self, advice: &MatchAdvice) -> Pick {
        match advice {
            MatchAdvice::Draw => self.clone(),
            MatchAdvice::Win => match self {
                Pick::Rock => Pick::Paper,
                Pick::Paper => Pick::Scissors,
                Pick::Scissors => Pick::Rock,
            },
            MatchAdvice::Loose => match self {
                Pick::Rock => Pick::Scissors,
                Pick::Paper => Pick::Rock,
                Pick::Scissors => Pick::Paper,
            },
        }
    }
}

fn score_matches(matches: &Vec<(Pick, Pick)>) -> i32 {
    return matches
        .iter()
        .map(|(p1, p2)| p2.score_against(&p1) + p2.worth())
        .sum();
}

fn main() {
    let input = fs::read_to_string("./input.txt").unwrap();

    let matches_part_1: Vec<(Pick, Pick)> = input
        .split("\n")
        .map(|match_input| {
            let picks = match_input.split_once(" ").unwrap();
            return (
                Pick::from_str(picks.0).unwrap(),
                Pick::from_str(picks.1).unwrap(),
            );
        })
        .collect();

    println!("Part1: {}", score_matches(&matches_part_1));

    let matches_part_2: Vec<(Pick, Pick)> = input
        .split("\n")
        .map(|match_input| {
            let picks = match_input.split_once(" ").unwrap();
            let p1 = Pick::from_str(picks.0).unwrap();
            let advice = MatchAdvice::from_str(picks.1).unwrap();
            let p2 = p1.pick_against_for_advice(&advice);

            return (p1, p2);
        })
        .collect();

    println!("Part2: {}", score_matches(&matches_part_2));
}
