#![allow(dead_code)]

use std::fmt::Display;

/// --- Day 1: Calorie Counting ---
pub struct DayOne {
    input: &'static str,
}

impl crate::Puzzle for DayOne {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        let split = DayOne::split_input(self.input);

        let mut i = 0;
        let mut result = 0;

        //  Loop over elves
        while i < split.len() {
            // Get the next elf
            let elf: Vec<u32> = split[i..split.len()]
                .iter()
                .map_while(|x| x.parse::<u32>().ok())
                .collect();
            i += elf.len() + 1;

            // Get the calories
            let sum: u32 = elf.iter().sum();
            if sum > result {
                result = sum;
            }
        }

        result.to_string()
    }

    fn part_two(&self) -> String {
        let split = DayOne::split_input(self.input);

        let mut i = 0;
        let mut result: Vec<u32> = vec![];

        // Loop over elves
        while i < split.len() {
            // Get the next elf
            let elf: Vec<u32> = split[i..split.len()]
                .iter()
                .map_while(|x| x.parse::<u32>().ok())
                .collect();
            i += elf.len() + 1;
            result.push(elf.iter().sum())
        }

        // Sort by most calories
        result.sort_by(|a, b| b.cmp(a));

        // Return the sum of the top 3
        result.iter().take(3).sum::<u32>().to_string()
    }
}

impl DayOne {
    fn split_input(input: &str) -> Vec<&str> {
        input
            .split_terminator("\n")
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
    }
}

impl Display for DayOne {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day One")
    }
}
