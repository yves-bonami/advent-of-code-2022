use std::fmt::Display;

/// --- Day 3: Rucksack Reorganization ---
pub struct DayThree {
    input: &'static str,
}

impl crate::Puzzle for DayThree {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        Self::split_input(self.input)
            .iter()
            .map(|r| r.split_at(r.len() / 2))
            .map(|(c1, c2)| c1.chars().find(|c| c2.contains(*c)).unwrap())
            .map(|c| match c.is_lowercase() {
                true => (c as u32) - 96,
                false => (c as u32) - 38,
            })
            .sum::<u32>()
            .to_string()
    }

    fn part_two(&self) -> String {
        Self::split_input(self.input)
            .chunks_exact(3)
            .map(|c| {
                if let [c1, c2, c3] = c {
                    c1.chars()
                        .find(|c| c2.contains(*c) && c3.contains(*c))
                        .unwrap()
                } else {
                    unreachable!()
                }
            })
            .map(|c| match c.is_lowercase() {
                true => (c as u32) - 96,
                false => (c as u32) - 38,
            })
            .sum::<u32>()
            .to_string()
    }
}

impl DayThree {
    fn split_input(input: &str) -> Vec<&str> {
        input
            .lines()
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
    }
}

impl Display for DayThree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Three")
    }
}
