use std::cmp;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn answer1() {
    let res:i32 = read_modules()
        .map(|x| (x / 3) - 2)
        .sum();
    println!("{}", res);
}

pub fn answer2() {
    let res:i32 = read_modules()
        .map(fuel)
        .sum();
    println!("{}", res);
}

fn fuel(module_mass : i32) -> i32 {
    let mut mass = module_mass;
    let mut required_fuel = 0;
    while mass > 0 {
        mass = cmp::max(mass / 3 - 2, 0);
        required_fuel += mass;
    }
    required_fuel
}

fn read_modules() -> impl Iterator<Item = i32>{
    let f = File::open("data/2019/day01.txt").unwrap();
    let fd = BufReader::new(f);
    fd.lines().map(|x| i32::from_str_radix(&x.unwrap(), 10).unwrap())
}
