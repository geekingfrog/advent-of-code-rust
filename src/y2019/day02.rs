use y2019::computer;

pub fn answer1() {
    let mut codes = computer::read_codes("data/2019/day02.txt");

    codes[1] = 12;
    codes[2] = 2;

    let mut computer = computer::Computer::new(codes);
    match computer.run() {
        Ok(_) => println!("{}", computer.codes[0]),
        Err(err) => panic!("error: {:?}", err),
    }
}

pub fn answer2() {
    let codes = computer::read_codes("data/2019/day02.txt");

    let target = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut current_codes = codes.clone();
            current_codes[1] = noun;
            current_codes[2] = verb;
            let mut computer = computer::Computer::new(current_codes);
            match computer.run() {
                Ok(_) => {
                    if computer.codes[0] == target {
                        println!("{}", noun * 100 + verb);
                        return;
                    }
                }
                Err(err) => panic!("error with noun: {}, verb {}: {:?}", noun, verb, err),
            }
        }
    }
    panic!("no solution found!")
}
