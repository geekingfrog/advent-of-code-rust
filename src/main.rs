#[macro_use]
extern crate lazy_static;

#[macro_use]
extern crate clap;
extern crate regex;

use clap::{App, Arg};

mod y2018;
mod y2019;

fn main() {
    let matches = App::new("Advent of code")
        .arg(Arg::with_name("year"))
        .arg(Arg::with_name("day"))
        .arg(Arg::with_name("pbNumber"))
        .get_matches();

    let year = value_t_or_exit!(matches, "year", String);
    let day = value_t_or_exit!(matches, "day", u8);
    let pb_number = value_t_or_exit!(matches, "pbNumber", u8);

    match year.as_ref() {
        "2018" => run2018(day, pb_number),
        "2019" => run2019(day, pb_number),
        _ => {
            println!("Unknown year: {}", year);
            std::process::exit(1);
        }
    }
}

fn run2018(day: u8, pb_number: u8) {
    match day * 10 + pb_number {
        11 => y2018::day01::answer1(),
        12 => y2018::day01::answer2(),
        21 => y2018::day02::answer1(),
        22 => y2018::day02::answer2(),
        _ => println!("Unknown pair day-pb number: {} - {}", day, pb_number),
    }
}

fn run2019(day: u8, pb_number: u8) {
    match day * 10 + pb_number {
        11 => y2019::day01::answer1(),
        12 => y2019::day01::answer2(),
        21 => y2019::day02::answer1(),
        22 => y2019::day02::answer2(),
        31 => y2019::day03::answer1(),
        32 => y2019::day03::answer2(),
        41 => y2019::day04::answer1(),
        42 => y2019::day04::answer2(),
        51 => y2019::day05::answer1(),
        52 => y2019::day05::answer2(),
        61 => y2019::day06::answer1(),
        62 => y2019::day06::answer2(),
        71 => y2019::day07::answer1(),
        72 => y2019::day07::answer2(),
        81 => y2019::day08::answer1(),
        82 => y2019::day08::answer2(),
        91 => y2019::day09::answer1(),
        92 => y2019::day09::answer2(),
        _ => println!("Unknown pair day-pb number: {} - {}", day, pb_number),
    }
}
