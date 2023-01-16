#[allow(unused_imports)]
#[macro_use] extern crate assert_matches;

mod grid;
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
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;

fn main() {
    let days = [
        crate::day1::aoc::day_main,
        crate::day2::aoc::day_main,
        crate::day3::aoc::day_main,
        crate::day4::aoc::day_main,
        crate::day5::aoc::day_main,
        crate::day6::aoc::day_main,
        crate::day7::aoc::day_main,
        crate::day8::aoc::day_main,
        crate::day9::aoc::day_main,
        crate::day10::aoc::day_main,
        crate::day11::aoc::day_main,
        crate::day12::aoc::day_main,
        crate::day13::aoc::day_main,
        crate::day14::aoc::day_main,
        crate::day15::aoc::day_main,
        crate::day16::aoc::day_main,
    ];

    match std::env::args().skip(1).next() {
        Some(arg) => {
            if let Some(day_str) = arg.strip_prefix("day") {
                if let Ok(day_num) = day_str.parse::<usize>() {
                    days[day_num]();
                    return;
                }
            }
            panic!("Expected first argument of form day<number> (e.g. day1), gut got {}", arg)
        },
        None => {
            days.last().unwrap()();
        }
    }
}
