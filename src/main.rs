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
        1 => solution!(DayOne("input/day_one.txt")),
        2 => solution!(DayTwo("input/day_two.txt")),
        3 => solution!(DayThree("input/day_three.txt")),
        4 => solution!(DayFour("input/day_four.txt")),
        5 => solution!(DayFive("input/day_five.txt")),
        6 => solution!(DaySix("input/day_six.txt")),
        7 => solution!(DaySeven("input/day_seven.txt")),
        8 => solution!(DayEight("input/day_eight.txt")),
        9 => solution!(DayNine("input/day_nine.txt")),
        10 => solution!(DayTen("input/day_ten.txt")),
        11 => solution!(DayEleven("input/day_eleven.txt")),
        12 => solution!(DayTwelve("input/day_twelve.txt")),
        13 => solution!(DayThirteen("input/day_thirteen.txt")),
        14 => solution!(DayFourteen("input/day_fourteen.txt")),
        15 => solution!(DayFifteen("input/day_fifteen.txt")),
        16 => solution!(DaySixteen("input/day_sixteen.txt")),
        17 => solution!(DaySeventeen("input/day_seventeen.txt")),
        18 => solution!(DayEighteen("input/day_eighteen.txt")),
        19 => solution!(DayNineteen("input/day_nineteen.txt")),
        20 => solution!(DayTwenty("input/day_twenty.txt")),
        21 => solution!(DayTwentyOne("input/day_twenty_one.txt")),
        22 => solution!(DayTwentyTwo("input/day_twenty_two.txt")),
        23 => solution!(DayTwentyThree("input/day_twenty_three.txt")),
        24 => solution!(DayTwentyFour("input/day_twenty_four.txt")),
        25 => println!("Merry Christmas!"),
        _ => println!("not a valid day"),
    };
}
