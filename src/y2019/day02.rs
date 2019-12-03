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
    let mut codes : Vec<usize> = buf
        .trim()
        .split(',')
        .map(|x| usize::from_str_radix(&x, 10).unwrap())
        .collect();

    codes[1] = 12;
    codes[2] = 2;
    let mut computer = Computer::new(codes);
    match computer.run() {
        Ok(result) => println!("{}", result),
        Err(err) => panic!("error: {:?}", err),
    }
}

pub fn answer2() {
    println!("{}", "coucou")
}
