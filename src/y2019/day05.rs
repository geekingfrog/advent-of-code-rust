use std::fs::File;
use std::io::Read;

pub fn answer1() {
    let codes = read_codes();
    let mut computer = Computer::new(codes, 1);
    computer.run().unwrap();
    println!("{:?}", computer.outputs.last().unwrap());
}

pub fn answer2() {
    let codes = read_codes();
    let mut computer = Computer::new(codes, 5);
    computer.run().unwrap();
    println!("{:?}", computer.outputs.last().unwrap());
}

struct Computer {
    instruction_idx: usize,
    codes: Vec<i32>,
    input: i32,
    outputs: Vec<i32>,
}

#[derive(Debug)]
enum ComputerError {
    OutOfBound,
    UnknownCode { code: i32 },
}

enum StepResult {
    Done,
    Continue,
}

#[derive(Debug, PartialEq, Eq)]
enum Mode {
    Position,
    Immediate,
}

impl Computer {
    fn new(codes: Vec<i32>, input: i32) -> Computer {
        Computer {
            instruction_idx: 0,
            codes: codes,
            input: input,
            outputs: Vec::new(),
        }
    }

    fn run(&mut self) -> Result<i32, ComputerError> {
        loop {
            let step_result = self.run_step()?;
            match step_result {
                StepResult::Done => break Ok(self.codes[0]),
                StepResult::Continue => continue,
            }
        }
    }

    fn run_step(&mut self) -> Result<StepResult, ComputerError> {
        if self.instruction_idx >= self.codes.len() {
            return Err(ComputerError::OutOfBound);
        }

        let code = self.codes[self.instruction_idx];
        let opcode = code % 100;
        // println!("{} - {:?}", self.instruction_idx, self.codes);
        // println!("code {}", code);
        match opcode {
            1 => self
                .binary_op(code, &|x, y| x + y)
                .map(|_| StepResult::Continue),
            2 => self
                .binary_op(code, &|x, y| x * y)
                .map(|_| StepResult::Continue),
            3 => {
                let addr = self.codes[self.instruction_idx + 1] as usize;
                self.codes[addr] = self.input;
                self.instruction_idx += 2;
                Ok(StepResult::Continue)
            }
            4 => {
                let output_val = self.get_val(code, 1);
                self.outputs.push(output_val);
                self.instruction_idx += 2;
                Ok(StepResult::Continue)
            }
            5 => {
                let val1 = self.get_val(code, 1);
                if val1 != 0 {
                    self.instruction_idx = self.get_val(code, 2) as usize;
                } else {
                    self.instruction_idx += 3;
                }
                Ok(StepResult::Continue)
            }
            6 => {
                let val1 = self.get_val(code, 1);
                if val1 == 0 {
                    self.instruction_idx = self.get_val(code, 2) as usize;
                } else {
                    self.instruction_idx += 3;
                }
                Ok(StepResult::Continue)
            }
            7 => {
                let val1 = self.get_val(code, 1);
                let val2 = self.get_val(code, 2);
                let res = if val1 < val2 { 1 } else { 0 };
                let addr = self.codes[self.instruction_idx + 3] as usize;
                self.codes[addr] = res;
                self.instruction_idx += 4;
                Ok(StepResult::Continue)
            }
            8 => {
                let val1 = self.get_val(code, 1);
                let val2 = self.get_val(code, 2);
                let res = if val1 == val2 { 1 } else { 0 };
                let addr = self.codes[self.instruction_idx + 3] as usize;
                self.codes[addr] = res;
                self.instruction_idx += 4;
                Ok(StepResult::Continue)
            }
            99 => Ok(StepResult::Done),
            c => Err(ComputerError::UnknownCode { code: c }),
        }
    }

    fn get_val(&mut self, code: i32, nth_param: i32) -> i32 {
        let mode = get_mode(code, nth_param);
        match mode {
            Mode::Position => {
                let i = self.instruction_idx + nth_param as usize;
                if i >= self.codes.len() {
                    panic!("BOOM Out of bound, codes {:?} - i {}", self.codes, i);
                }
                self.codes[self.codes[i] as usize]
            }
            Mode::Immediate => self.codes[self.instruction_idx + nth_param as usize],
        }
    }

    fn binary_op(&mut self, code: i32, f: &dyn Fn(i32, i32) -> i32) -> Result<(), ComputerError> {
        if self.instruction_idx + 3 >= self.codes.len() {
            return Err(ComputerError::OutOfBound);
        }

        let val1 = self.get_val(code, 1);
        let val2 = self.get_val(code, 2);
        let idx_res = self.codes[self.instruction_idx + 3] as usize;
        let result = f(val1, val2);
        self.codes[idx_res] = result;
        self.instruction_idx = self.instruction_idx + 4;
        Ok(())
    }
}

fn get_mode(code: i32, nth_param: i32) -> Mode {
    let base: usize = 10;
    let i = (code as usize) / base.pow((nth_param + 1) as u32) % 10;
    match i {
        0 => Mode::Position,
        1 => Mode::Immediate,
        _ => panic!(
            "Cannot get mode for code {} and nth_param {}",
            code, nth_param
        ),
    }
}

fn read_codes() -> Vec<i32> {
    let mut f = File::open("data/2019/day05.txt").unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf.trim()
        .split(',')
        .map(|x| i32::from_str_radix(&x, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mode() {
        assert_eq!(get_mode(1002, 1), Mode::Position);
        assert_eq!(get_mode(1002, 2), Mode::Immediate);
        assert_eq!(get_mode(1002, 3), Mode::Position);
    }

    #[test]
    fn test_larger_example1() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test, 1);
        c.run().unwrap();
        assert_eq!(c.outputs.last().unwrap(), &999);
    }

    #[test]
    fn test_larger_example2() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test, 8);
        c.run().unwrap();
        assert_eq!(c.outputs.last().unwrap(), &1000);
    }

    #[test]
    fn test_larger_example3() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test, 10);
        c.run().unwrap();
        assert_eq!(c.outputs.last().unwrap(), &1001);
    }
}
