#![allow(dead_code)]

use std::fmt::Display;

pub struct DaySeventeen {
    input: &'static str,
}

impl crate::Puzzle for DaySeventeen {
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

impl Display for DaySeventeen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Seventeen")
    }
}
