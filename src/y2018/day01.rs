use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashSet;

pub fn answer1() {
    let freqs = get_frequencies();
    let res : i32 = freqs.iter().sum();
    println!("{}", res);
}

pub fn answer2() {
    let freqs = get_frequencies();
    let mut seen : HashSet<i32> = HashSet::new();
    let mut i = 0;
    let mut current : i32 = 0;
    let l = freqs.len();

    while !seen.contains(&current) {
        seen.insert(current);
        if i >= l {
            i = 0;
        }
        current = current + freqs[i];
        i = i+1;
    }
    println!("{}", current);
}

fn get_frequencies() -> Vec<i32> {
    let f = File::open("data/2018/day01.txt").unwrap();
    let fd = BufReader::new(&f);
    fd.lines().map(|x| i32::from_str_radix(&x.unwrap(), 10).unwrap()).collect()
}
