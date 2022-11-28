use aoc2022::prelude::*;
use clap::Parser;

macro_rules! solution {
    ($day:ident($input:literal)) => {{
        let input = include_str!($input);
        let puzzle = $day::new(input);

        println!("Solution for {}", puzzle);
        println!("Part One: {}", puzzle.part_one());
        println!("Part Two: {}", puzzle.part_two());
    }};
}

#[derive(clap::Parser)]
struct Args {
    day: u8,
}

fn main() {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    match args.day {
        1 => solution!(DayOne("input/day_01.txt")),
        2 => solution!(DayTwo("input/day_02.txt")),
        3 => solution!(DayThree("input/day_03.txt")),
        4 => solution!(DayFour("input/day_04.txt")),
        5 => solution!(DayFive("input/day_05.txt")),
        6 => solution!(DaySix("input/day_06.txt")),
        7 => solution!(DaySeven("input/day_07.txt")),
        8 => solution!(DayEight("input/day_08.txt")),
        9 => solution!(DayNine("input/day_09.txt")),
        10 => solution!(DayTen("input/day_10.txt")),
        11 => solution!(DayEleven("input/day_11.txt")),
        12 => solution!(DayTwelve("input/day_12.txt")),
        13 => solution!(DayThirteen("input/day_13.txt")),
        14 => solution!(DayFourteen("input/day_14.txt")),
        15 => solution!(DayFifteen("input/day_15.txt")),
        16 => solution!(DaySixteen("input/day_16.txt")),
        17 => solution!(DaySeventeen("input/day_17.txt")),
        18 => solution!(DayEighteen("input/day_18.txt")),
        19 => solution!(DayNineteen("input/day_19.txt")),
        20 => solution!(DayTwenty("input/day_20.txt")),
        21 => solution!(DayTwentyOne("input/day_21.txt")),
        22 => solution!(DayTwentyTwo("input/day_22.txt")),
        23 => solution!(DayTwentyThree("input/day_23.txt")),
        24 => solution!(DayTwentyFour("input/day_24.txt")),
        25 => println!("Merry Christmas!"),
        _ => println!("not a valid day"),
    };
}
