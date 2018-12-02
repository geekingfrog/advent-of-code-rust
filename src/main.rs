#[macro_use]
extern crate clap;

use clap::{Arg, App};

mod y2018;

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
        _ => println!("Unknown pair day-pb number: {} - {}", day, pb_number),
    }
}
