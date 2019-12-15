use std::fs::File;
use std::io::Read;

#[derive(Debug)]
pub struct Computer {
    pub instruction_idx: usize,
    pub codes: Vec<i64>,
    inputs: Vec<i64>,
    input_idx: usize,
    pub outputs: Vec<i64>,
    relative_base: i64,
}

#[derive(Debug)]
pub enum ComputerError {
    UnknownCode { code: i64 },
    MissingInput,
}

enum StepResult {
    Done,
    AwaitInput,
    Output(i64),
    Continue,
}

pub enum RunResult {
    Done,
    AwaitInput,
    Output(i64),
}

#[derive(Debug, PartialEq, Eq)]
pub enum Mode {
    Position,
    Immediate,
    Relative,
}

impl Computer {
    pub fn new(codes: Vec<i64>) -> Computer {
        Computer {
            instruction_idx: 0,
            codes: codes,
            inputs: Vec::new(),
            input_idx: 0,
            outputs: Vec::new(),
            relative_base: 0,
        }
    }

    pub fn with_input(&mut self, inputs: Vec<i64>) {
        self.inputs = inputs;
    }

    pub fn run(&mut self) -> Result<RunResult, ComputerError> {
        loop {
            let step_result = self.run_step()?;
            match step_result {
                StepResult::Done => break Ok(RunResult::Done),
                StepResult::AwaitInput => break Ok(RunResult::AwaitInput),
                StepResult::Continue => continue,
                StepResult::Output(x) => break Ok(RunResult::Output(x)),
            }
        }
    }

    pub fn run_until_halt(&mut self) -> Result<RunResult, ComputerError> {
        loop {
            let step_result = self.run_step()?;
            match step_result {
                StepResult::Done => break Ok(RunResult::Done),
                StepResult::AwaitInput => break Err(ComputerError::MissingInput),
                StepResult::Continue => continue,
                StepResult::Output(_) => continue,
            }
        }
    }

    pub fn run_with_inputs(&mut self, inputs: Vec<i64>) -> Result<RunResult, ComputerError> {
        self.inputs.extend_from_slice(&inputs);
        self.run()
    }

    fn run_step(&mut self) -> Result<StepResult, ComputerError> {
        let code = self.get_at_mem(self.instruction_idx);
        let opcode = code % 100;
        // println!("run step with code: {}", code);

        match opcode {
            1 => self
                .binary_op(code, &|x, y| x + y)
                .map(|_| StepResult::Continue),

            2 => self
                .binary_op(code, &|x, y| x * y)
                .map(|_| StepResult::Continue),

            3 => {
                let addr = self.get_val(code, 1) as usize;
                // let addr = self.codes[self.instruction_idx + 1] as usize;
                if self.input_idx >= self.inputs.len() {
                    Ok(StepResult::AwaitInput)
                } else {
                    self.set_at_mem(addr, self.inputs[self.input_idx]);
                    self.input_idx += 1;
                    self.instruction_idx += 2;
                    Ok(StepResult::Continue)
                }
            }

            4 => {
                let output_val = self.get_val(code, 1);
                self.outputs.push(output_val);
                self.instruction_idx += 2;
                Ok(StepResult::Output(output_val))
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
                let addr = self.get_at_mem(self.instruction_idx + 3) as usize;
                self.codes[addr] = res;
                self.instruction_idx += 4;
                Ok(StepResult::Continue)
            }

            8 => {
                let val1 = self.get_val(code, 1);
                let val2 = self.get_val(code, 2);
                let res = if val1 == val2 { 1 } else { 0 };
                let addr = self.get_at_mem(self.instruction_idx + 3) as usize;
                self.set_at_mem(addr, res);
                self.instruction_idx += 4;
                Ok(StepResult::Continue)
            }

            9 => {
                let delta = self.get_val(code, 1);
                self.relative_base += delta;
                self.instruction_idx += 2;
                // println!("new base {}", self.relative_base);
                Ok(StepResult::Continue)
            }

            99 => Ok(StepResult::Done),

            c => Err(ComputerError::UnknownCode { code: c }),
        }
    }

    fn get_at_mem(&mut self, i: usize) -> i64 {
        if i >= self.codes.len() {
            // println!("get and resizing to {}", i+4);
            self.codes.resize(i + 4, 0);
        }
        self.codes[i]
    }

    fn set_at_mem(&mut self, i: usize, val: i64) {
        if i >= self.codes.len() {
            // println!("set and resizing to {}", i+4);
            self.codes.resize(i + 4, 0);
        }
        self.codes[i] = val;
    }

    fn get_val(&mut self, code: i64, nth_param: usize) -> i64 {
        let mode = get_mode(code, nth_param);
        // println!("getting val with mode {} and pos {}", code, nth_param);
        match mode {
            Mode::Position => {
                let i = self.instruction_idx + nth_param;
                let addr = self.get_at_mem(i);
                self.get_at_mem(addr as usize)
            }
            Mode::Immediate => self.get_at_mem(self.instruction_idx + nth_param),
            Mode::Relative => {
                let i = self.instruction_idx + nth_param;
                // println!(
                //     "get val relative for code {} and param {} with base {} and i={}",
                //     code, nth_param, self.relative_base, i
                // );
                let addr = self.get_at_mem(i);
                self.get_at_mem((addr + self.relative_base) as usize)
            }
        }
    }

    fn binary_op(&mut self, code: i64, f: &dyn Fn(i64, i64) -> i64) -> Result<(), ComputerError> {
        let val1 = self.get_val(code, 1);
        let val2 = self.get_val(code, 2);
        let idx_res = self.get_at_mem(self.instruction_idx + 3) as usize;
        let result = f(val1, val2);
        self.set_at_mem(idx_res, result);
        self.instruction_idx = self.instruction_idx + 4;
        Ok(())
    }
}

pub fn get_mode(code: i64, nth_param: usize) -> Mode {
    let base: usize = 10;
    let i = (code as usize) / base.pow((nth_param + 1) as u32) % 10;
    match i {
        0 => Mode::Position,
        1 => Mode::Immediate,
        2 => Mode::Relative,
        _ => panic!(
            "Cannot get mode for code {} and nth_param {}",
            code, nth_param
        ),
    }
}

pub fn read_codes(file_path: &str) -> Vec<i64> {
    let mut f = File::open(file_path).unwrap();
    let mut buf = String::new();
    f.read_to_string(&mut buf).unwrap();
    buf.trim()
        .split(',')
        .map(|x| i64::from_str_radix(&x, 10).unwrap())
        .collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computer_get_mode() {
        assert_eq!(get_mode(1002, 1), Mode::Position);
        assert_eq!(get_mode(1002, 2), Mode::Immediate);
        assert_eq!(get_mode(1002, 3), Mode::Position);
    }

    #[test]
    fn test_computer_larger_example1() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test);
        c.with_input(vec![1]);
        c.run_until_halt().unwrap();
        assert_eq!(c.outputs.last().unwrap(), &999);
    }

    #[test]
    fn test_computer_larger_example2() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test);
        c.with_input(vec![8]);
        c.run().unwrap();
        assert_eq!(c.outputs.last().unwrap(), &1000);
    }

    #[test]
    fn test_computer_larger_example3() {
        let larger_test = vec![
            3, 21, 1008, 21, 8, 20, 1005, 20, 22, 107, 8, 21, 20, 1006, 20, 31, 1106, 0, 36, 98, 0,
            0, 1002, 21, 125, 20, 4, 20, 1105, 1, 46, 104, 999, 1105, 1, 46, 1101, 1000, 1, 20, 4,
            20, 1105, 1, 46, 98, 99,
        ];
        let mut c = Computer::new(larger_test);
        c.run_with_inputs(vec![10]).unwrap();
        assert_eq!(c.outputs.last().unwrap(), &1001);
    }
}
