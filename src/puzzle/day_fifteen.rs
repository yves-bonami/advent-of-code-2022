#![allow(dead_code)]

use std::fmt::Display;

pub struct DayFifteen {
    input: &'static str,
}

impl crate::Puzzle for DayFifteen {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        todo!()
    }

    fn part_two(&self) -> String {
        todo!()
    }
}

impl Display for DayFifteen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Fifteen")
    }
}
