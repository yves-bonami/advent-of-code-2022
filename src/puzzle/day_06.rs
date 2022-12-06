use std::{collections::HashSet, fmt::Display};

pub struct DaySix {
    input: &'static str,
}

impl crate::Puzzle for DaySix {
    fn new(input: &'static str) -> Self {
        Self { input }
    }

    fn part_one(&self) -> String {
        DaySix::search(self.input, 4).to_string()
    }

    fn part_two(&self) -> String {
        DaySix::search(self.input, 14).to_string()
    }
}

impl DaySix {
    fn search(input: &str, n: usize) -> usize {
        input
            .as_bytes()
            .windows(n)
            .enumerate()
            .skip_while(|(_, c)| {
                let mut set = HashSet::new();
                c.into_iter().all(|x| set.insert(x));
                set.len() != n
            })
            .nth(1)
            .unwrap()
            .0
            + n
            - 1
    }
}

impl Display for DaySix {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Six")
    }
}
