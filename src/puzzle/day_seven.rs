#![allow(dead_code)]

use std::fmt::Display;

pub struct DaySeven {
    input: &'static str,
}

impl crate::Puzzle for DaySeven {
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

impl Display for DaySeven {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Seven")
    }
}
