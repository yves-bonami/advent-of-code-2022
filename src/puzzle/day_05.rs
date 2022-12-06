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
        if let [crates, instructions] = self
            .input
            .lines()
            .collect::<Vec<&str>>()
            .split(|l| l.is_empty())
            .collect::<Vec<&[&str]>>()[..]
        {
            let crates = crates[0..crates.len() - 1]
                .iter()
                .map(|c| {
                    c.chars()
                        .collect::<Vec<char>>()
                        .chunks(4)
                        .map(|c| c[1])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            let mut crates = DayFive::transpose(crates)
                .into_iter()
                .map(|c| c.into_iter().filter(|x| *x != ' ').collect::<Vec<char>>())
                .map(|mut c| {
                    c.reverse();
                    c
                })
                .collect::<Vec<Vec<char>>>();

            instructions.iter().for_each(|i| {
                if let [_, number, _, source, _, target] =
                    i.trim().split_whitespace().collect::<Vec<&str>>()[..]
                {
                    let number: u32 = number.parse().unwrap();
                    let source: usize = source.parse::<usize>().unwrap() - 1;
                    let target: usize = target.parse::<usize>().unwrap() - 1;

                    for _ in 0..number {
                        let source = crates.get_mut(source).unwrap();
                        let value = source.pop().unwrap();
                        let target = crates.get_mut(target).unwrap();
                        target.push(value);
                    }
                }
            });

            crates.iter().fold("".to_string(), |accum, c| {
                format!("{}{}", accum, c.last().unwrap().to_string())
            })
        } else {
            unreachable!()
        }
    }

    fn part_two(&self) -> String {
        if let [crates, instructions] = self
            .input
            .lines()
            .collect::<Vec<&str>>()
            .split(|l| l.is_empty())
            .collect::<Vec<&[&str]>>()[..]
        {
            let crates = crates[0..crates.len() - 1]
                .iter()
                .map(|c| {
                    c.chars()
                        .collect::<Vec<char>>()
                        .chunks(4)
                        .map(|c| c[1])
                        .collect::<Vec<char>>()
                })
                .collect::<Vec<Vec<char>>>();

            let mut crates = DayFive::transpose(crates)
                .into_iter()
                .map(|c| c.into_iter().filter(|x| *x != ' ').collect::<Vec<char>>())
                .map(|mut c| {
                    c.reverse();
                    c
                })
                .collect::<Vec<Vec<char>>>();

            instructions.iter().for_each(|i| {
                if let [_, number, _, source, _, target] =
                    i.trim().split_whitespace().collect::<Vec<&str>>()[..]
                {
                    let number: usize = number.parse().unwrap();
                    let source: usize = source.parse::<usize>().unwrap() - 1;
                    let target: usize = target.parse::<usize>().unwrap() - 1;

                    let stack = crates.get_mut(source).unwrap();
                    let mut crates_to_move =
                        stack.drain((stack.len() - number)..).collect::<Vec<char>>();
                    crates.get_mut(target).unwrap().append(&mut crates_to_move);
                }
            });

            crates.iter().fold("".to_string(), |accum, c| {
                format!("{}{}", accum, c.last().unwrap().to_string())
            })
        } else {
            unreachable!()
        }
    }
}

impl DayFive {
    fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>>
    where
        T: Clone,
    {
        (0..v[0].len())
            .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
            .collect()
    }
}

impl Display for DayFive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Day Five")
    }
}
