use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::collections::HashMap;
use std::hash::Hash;

pub fn answer1() {
    let ids = get_ids();
    let counts = ids.iter().map(|l| {
        let mut m = HashMap::new();
        for c in l.as_str().chars() {
            m.entry(c)
                .and_modify(|x| {*x += 1})
                .or_insert(1);
        }
        m
    });

    let mut has_3 = 0;
    let mut has_2 = 0;
    for id in counts {
        if has_value(&id, 3) { has_3 += 1; }
        if has_value(&id, 2) { has_2 += 1; }
    }

    println!("{}", has_3 * has_2);
}

pub fn answer2() {
    let ids = get_ids();

    for (i, x) in ids.iter().enumerate() {
        let l = x.len();
        for y in ids.iter().skip(i + 1) {
            let common = common_letters(x, y);
            if common.len() == l - 1 {
                println!("{}", common);
                return;
            }
        }
    }
    unreachable!();
}

fn get_ids() -> Vec<String> {
    let f = File::open("data/2018/day02.txt").unwrap();
    let fd = BufReader::new(&f);
    fd.lines()
        .map(|x| x.unwrap())
        .collect()
}

fn has_value<K: Eq + Hash, V: Eq>(m : &HashMap<K, V>, x: V) -> bool {
    m.values()
        .find(|v| *v == &x)
        .is_some()
}

fn common_letters(x: &String, y: &String) -> String {
    x.chars().zip(y.chars())
        .filter(|(c1, c2)| c1 == c2)
        .map(|x| x.0)
        .collect()
}
