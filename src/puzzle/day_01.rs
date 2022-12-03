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
        let split = Self::split_input(self.input);
        let mut elves: Vec<u32> = split
            .split(|x| x.is_empty())
            .map(|x| x.iter().map(|c| c.parse::<u32>().unwrap()).sum())
            .collect();
        elves.sort_by(|a, b| b.cmp(a));
        elves.first().unwrap().to_string()
    }

    fn part_two(&self) -> String {
        let split = Self::split_input(self.input);
        let mut elves: Vec<u32> = split
            .split(|x| x.is_empty())
            .map(|x| x.iter().map(|c| c.parse::<u32>().unwrap()).sum())
            .collect();
        elves.sort_by(|a, b| b.cmp(a));
        elves.iter().take(3).sum::<u32>().to_string()
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
