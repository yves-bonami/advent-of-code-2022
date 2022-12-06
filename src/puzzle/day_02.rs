use std::{fmt::Display, str::FromStr};

/// --- Day 2: Rock Paper Scissors ---
pub struct DayTwo {
    input: &'static str,
}

impl crate::Puzzle for DayTwo {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        Self::split_input(self.input)
            .iter()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(elf, you)| (Shape::from_str(elf).unwrap(), Shape::from_str(you).unwrap()))
            .map(|(elf, you)| elf.play(you))
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        Self::split_input(self.input)
            .iter()
            .map(|line| line.split_once(' ').unwrap())
            .map(|(elf, you)| {
                (
                    Shape::from_str(elf).unwrap(),
                    Outcome::from_str(you).unwrap(),
                )
            })
            .map(|(elf, you)| elf.play_from_outcome(you))
            .sum::<u32>()
            .to_string()
    }
}

impl DayTwo {
    fn split_input(input: &str) -> Vec<&str> {
        input
            .lines()
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
    }
}

impl Display for DayTwo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Two")
    }
}

#[derive(Eq, PartialEq)]
enum Shape {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Shape {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" | "X" => Self::Rock,
            "B" | "Y" => Self::Paper,
            "C" | "Z" => Self::Scissors,
            _ => unreachable!(),
        })
    }
}

impl Shape {
    fn play(&self, other: Shape) -> u32 {
        let mut points = match other {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        };

        points += match (self, other) {
            (Shape::Rock, Shape::Rock) => 3,
            (Shape::Rock, Shape::Paper) => 6,
            (Shape::Rock, Shape::Scissors) => 0,
            (Shape::Paper, Shape::Rock) => 0,
            (Shape::Paper, Shape::Paper) => 3,
            (Shape::Paper, Shape::Scissors) => 6,
            (Shape::Scissors, Shape::Rock) => 6,
            (Shape::Scissors, Shape::Paper) => 0,
            (Shape::Scissors, Shape::Scissors) => 3,
        };

        points
    }

    fn play_from_outcome(&self, outcome: Outcome) -> u32 {
        let shape = match (self, outcome) {
            (Shape::Rock, Outcome::Win) => Shape::Paper,
            (Shape::Rock, Outcome::Draw) => Shape::Rock,
            (Shape::Rock, Outcome::Lose) => Shape::Scissors,
            (Shape::Paper, Outcome::Win) => Shape::Scissors,
            (Shape::Paper, Outcome::Draw) => Shape::Paper,
            (Shape::Paper, Outcome::Lose) => Shape::Rock,
            (Shape::Scissors, Outcome::Win) => Shape::Rock,
            (Shape::Scissors, Outcome::Draw) => Shape::Scissors,
            (Shape::Scissors, Outcome::Lose) => Shape::Paper,
        };
        self.play(shape)
    }
}

enum Outcome {
    Win,
    Draw,
    Lose,
}

impl FromStr for Outcome {
    type Err = Box<dyn std::error::Error>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "X" => Self::Lose,
            "Y" => Self::Draw,
            "Z" => Self::Win,
            _ => unreachable!(),
        })
    }
}
