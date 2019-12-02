use std::fs::File;
// use std::io::BufRead;
use std::io::Read;
// use std::io::BufReader;

pub fn answer1() {
    let mut f = File::open("data/2019/day02.txt").unwrap();
    // let fd = BufReader::new(f);
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut codes : Vec<usize> = buf
        .trim()
        .split(',')
        .map(|x| usize::from_str_radix(&x, 10).unwrap())
        .collect();

    codes[1] = 12;
    codes[2] = 2;

    let mut idx = 0;
    loop {
        if idx >= codes.len() { break; }
        let ins = codes[idx];
        match ins {
            1 => {
                let idx1 = codes[idx+1];
                let idx2 = codes[idx+2];
                let idx_res = codes[idx+3];
                let result = codes[idx1] + codes[idx2];
                codes[idx_res] = result;
            }
            2 => {
                let idx1 = codes[idx+1];
                let idx2 = codes[idx+2];
                let idx_res = codes[idx+3];
                let result = codes[idx1] * codes[idx2];
                codes[idx_res] = result;
            }
            99 => break,
            _ => panic!("Unknown code: {}", ins)
        }
        idx = idx + 4;
    }

    println!("{}", codes[0])
}

pub fn answer2() {
    println!("{}", "coucou")
}
