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
    println!("coucou");
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
        computer
            .run_with_inputs(vec![*phase_setting, input])
            .unwrap();
        computer.outputs[0]
    })
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
}
