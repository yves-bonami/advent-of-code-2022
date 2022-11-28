#![allow(dead_code)]

use std::fmt::Display;

pub struct DayThree {
    input: &'static str,
}

impl crate::Puzzle for DayThree {
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

impl Display for DayThree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Three")
    }
}
