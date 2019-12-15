use y2019::computer;

pub fn answer1() {
    let codes = computer::read_codes("data/2019/day07.txt");
    let result = permutations(0, 5)
        .iter()
        .map(|p| run_sequence(&codes, p))
        .max()
        .unwrap();
    println!("{}", result);
}

pub fn answer2() {
    let codes = computer::read_codes("data/2019/day07.txt");
    let result = permutations(5, 10)
        .iter()
        .map(|p| run_sequence2(&codes, p))
        .max()
        .unwrap();
    println!("{}", result);
}

fn permutations(start: i32, end: i32) -> Vec<Vec<i32>> {
    // gruiiiiik
    let mut res = Vec::new();
    for a in start..end {
        for b in start..end {
            if b == a {
                continue;
            }
            for c in start..end {
                if c == b || c == a {
                    continue;
                }
                for d in start..end {
                    if d == c || d == b || d == a {
                        continue;
                    }
                    for e in start..end {
                        if e == d || e == c || e == b || e == a {
                            continue;
                        }
                        res.push(vec![a, b, c, d, e]);
                    }
                }
            }
        }
    }
    res
}

fn run_sequence(codes: &Vec<i32>, sequence: &Vec<i32>) -> i32 {
    sequence.iter().fold(0, |input, phase_setting| {
        let mut computer = computer::Computer::new(codes.clone());
        let res = computer
            .run_with_inputs(vec![*phase_setting, input])
            .unwrap();
        match res {
            computer::RunResult::Done => panic!("should output something before halting"),
            computer::RunResult::Output(x) => x,
            computer::RunResult::AwaitInput => panic!("oops, that should terminate"),
        }
    })
}

fn run_sequence2(codes: &Vec<i32>, sequence: &Vec<i32>) -> i32 {
    let mut computers: Vec<Box<computer::Computer>> = sequence
        .iter()
        .map(|i| {
            let mut c = computer::Computer::new(codes.clone());
            c.with_initial_input(vec![*i]);
            Box::new(c)
        })
        .collect();

    let mut start = 0;
    let mut done = false;
    while !done {
        (0..computers.len()).fold(start, |input, idx| {
            let c = &mut computers[idx];
            match c.run_with_inputs(vec![input]).unwrap() {
                computer::RunResult::Done => {
                    done = true;
                    start
                }
                computer::RunResult::AwaitInput => {
                    panic!("wait input but nothing available");
                }
                computer::RunResult::Output(x) => {
                    start = x;
                    x
                }
            }
        });
    }
    start
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_thruster1() {
        let codes = vec![
            3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0,
        ];
        assert_eq!(run_sequence(&codes, &vec![4, 3, 2, 1, 0]), 43210);
    }

    #[test]
    fn test_thruster2() {
        let codes = vec![
            3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23, 23, 4, 23,
            99, 0, 0,
        ];
        assert_eq!(run_sequence(&codes, &vec![0, 1, 2, 3, 4]), 54321);
    }

    #[test]
    fn test_thruster3() {
        let codes = vec![
            3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7, 33, 1,
            33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0,
        ];
        assert_eq!(run_sequence(&codes, &vec![1, 0, 4, 3, 2]), 65210);
    }

    #[test]
    fn test_thruster4() {
        let codes = vec![
            3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001, 28, -1,
            28, 1005, 28, 6, 99, 0, 0, 5,
        ];
        assert_eq!(run_sequence2(&codes, &vec![9, 8, 7, 6, 5]), 139629729);
    }

    #[test]
    fn test_thruster5() {
        let codes = vec![
            3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26, 1001, 54,
            -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55, 2, 53, 55, 53, 4,
            53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10,
        ];
        assert_eq!(run_sequence2(&codes, &vec![9, 7, 8, 5, 6]), 18216);
    }
}
