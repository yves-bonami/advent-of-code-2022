#![allow(dead_code)]

use std::fmt::Display;

pub struct DayFive {
    input: &'static str,
}

impl crate::Puzzle for DayFive {
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

impl Display for DayFive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Five")
    }
}
