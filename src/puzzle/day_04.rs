use std::fmt::Display;

pub struct DayFour {
    input: &'static str,
}

impl crate::Puzzle for DayFour {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        Self::split_input(self.input)
            .iter()
            .map(|x| {
                x.split(&['-', ','][..])
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .fold(0, |acc, x| {
                if (x[0] >= x[2] && x[1] <= x[3]) || (x[0] <= x[2] && x[1] >= x[3]) {
                    acc + 1
                } else {
                    acc
                }
            })
            .to_string()
    }

    fn part_two(&self) -> String {
        Self::split_input(self.input)
            .iter()
            .map(|x| {
                x.split(&['-', ','][..])
                    .map(|c| c.parse::<u32>().unwrap())
                    .collect::<Vec<u32>>()
            })
            .fold(0, |acc, x| {
                if (x[0] <= x[3] && x[1] >= x[2]) || (x[0] >= x[2] && x[1] <= x[3]) {
                    acc + 1
                } else {
                    acc
                }
            })
            .to_string()
    }
}

impl DayFour {
    fn split_input(input: &str) -> Vec<&str> {
        input
            .lines()
            .into_iter()
            .map(|x| x.trim())
            .collect::<Vec<&str>>()
    }
}

impl Display for DayFour {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Four")
    }
}
