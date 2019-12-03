use std::fs::File;
use std::io::Read;

struct Computer {
    instruction_idx: usize,
    codes: Vec<usize>,
}

#[derive(Debug)]
enum ComputerError {
    OutOfBound,
    UnknownCode { code: usize },
}

enum StepResult {
    Done,
    Continue,
}

impl Computer {
    fn new(codes: Vec<usize>) -> Computer {
        Computer { instruction_idx: 0, codes: codes}
    }

    fn run(&mut self) -> Result<usize, ComputerError> {
        loop {
            let step_result = self.run_step()?;
            match step_result {
                StepResult::Done => break Ok(self.codes[0]),
                StepResult::Continue => continue,
            }
        }
    }

    fn run_step(&mut self) -> Result<StepResult, ComputerError> {
        if self.instruction_idx >= self.codes.len()
        {
            return Err(ComputerError::OutOfBound);
        }

        match self.codes[self.instruction_idx] {
            1 => self.bin_op(&|x,y| x+y).map(|_| StepResult::Continue),
            2 => self.bin_op(&|x,y| x*y).map(|_| StepResult::Continue),
            99 => Ok(StepResult::Done),
            c => Err(ComputerError::UnknownCode { code: c })
        }
    }

    fn bin_op(&mut self, f: &dyn Fn(usize, usize) -> usize) -> Result<(), ComputerError>{
        if self.instruction_idx + 3 >= self.codes.len()
        {
            return Err(ComputerError::OutOfBound);
        }
        let idx1 = self.codes[self.instruction_idx+1];
        let idx2 = self.codes[self.instruction_idx+2];
        let idx_res = self.codes[self.instruction_idx+3];
        let result = f(self.codes[idx1], self.codes[idx2]);
        self.codes[idx_res] = result;
        self.instruction_idx = self.instruction_idx + 4;
        Ok(())
    }
}

pub fn answer1() {
    let mut f = File::open("data/2019/day02.txt").unwrap();
    // let fd = BufReader::new(f);
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    let mut codes = read_codes();

    codes[1] = 12;
    codes[2] = 2;

    let mut computer = Computer::new(codes);
    match computer.run() {
        Ok(result) => println!("{}", result),
        Err(err) => panic!("error: {:?}", err),
    }
}

pub fn answer2() {
    let codes = read_codes();

    let target = 19690720;
    for noun in 0..99 {
        for verb in 0..99 {
            let mut current_codes = codes.clone();
            current_codes[1] = noun;
            current_codes[2] = verb;
            let mut computer = Computer::new(current_codes);
            match computer.run() {
                Ok(result) => {
                    if result == target {
                        println!("{}", noun * 100 + verb);
                        return;
                    }
                },
                Err(err) => panic!("error with noun: {}, verb {}: {:?}", noun, verb, err),
            }
        }
    }
    panic!("no solution found!")
}

fn read_codes() -> Vec<usize> {
    let mut f = File::open("data/2019/day02.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf.trim()
        .split(',')
        .map(|x| usize::from_str_radix(&x, 10).unwrap())
        .collect()
}
