#[allow(unused_imports)]
#[macro_use] extern crate assert_matches;

mod util;
mod parser;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod day10;
mod day11;

use crate::day11::aoc;

fn main() {
    aoc::day_main().unwrap();
}
