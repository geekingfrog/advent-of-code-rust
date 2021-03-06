use std::collections::HashSet;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn answer1() {
    let freqs = get_frequencies();
    let res: i32 = freqs.iter().sum();
    println!("{}", res);
}

pub fn answer2() {
    let freqs = get_frequencies();
    let mut seen: HashSet<i32> = HashSet::new();
    let mut current: i32 = 0;

    for f in freqs.iter().cycle() {
        if seen.contains(&current) {
            println!("{}", current);
            return;
        }
        seen.insert(current);
        current += f;
    }
}

fn get_frequencies() -> Vec<i32> {
    let f = File::open("data/2018/day01.txt").unwrap();
    let fd = BufReader::new(&f);
    fd.lines()
        .map(|x| i32::from_str_radix(&x.unwrap(), 10).unwrap())
        .collect()
}
