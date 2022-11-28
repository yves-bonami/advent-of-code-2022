#![allow(dead_code)]

use std::fmt::Display;

pub struct DayTwentyFour {
    input: &'static str,
}

impl crate::Puzzle for DayTwentyFour {
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

impl Display for DayTwentyFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day TwentyFour")
    }
}
