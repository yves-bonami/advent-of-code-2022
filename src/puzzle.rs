pub mod day_one;
pub mod day_two;
pub mod day_three;
pub mod day_four;
pub mod day_five;
pub mod day_six;
pub mod day_seven;
pub mod day_eight;
pub mod day_nine;
pub mod day_ten;
pub mod day_eleven;
pub mod day_twelve;
pub mod day_thirteen;
pub mod day_fourteen;
pub mod day_fifteen;
pub mod day_sixteen;
pub mod day_seventeen;
pub mod day_eighteen;
pub mod day_nineteen;
pub mod day_twenty;
pub mod day_twenty_one;
pub mod day_twenty_two;
pub mod day_twenty_three;
pub mod day_twenty_four;

pub trait Puzzle: std::fmt::Display {
    fn new(input: &'static str) -> Self;
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;
}