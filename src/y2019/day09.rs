use y2019::computer;

pub fn answer1() {
    // let codes = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
    // let codes = vec![1102,34915192,34915192,7,4,7,99,0];
    let codes = computer::read_codes("data/2019/day09.txt");
    let mut c = computer::Computer::new(codes);
    c.with_input(vec![1]);
    loop {
        match c.run().unwrap() {
            computer::RunResult::Done => break,
            computer::RunResult::AwaitInput => panic!("await input but shouldn't happen"),
            computer::RunResult::Output(_) => (),
        }
    }
    println!("{:?}", c.outputs);
}

pub fn answer2() {
    println!("coucou");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computer_quine() {
        let codes = vec![109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99];
        let mut c = computer::Computer::new(codes.clone());
        c.run_until_halt().unwrap();
        assert_eq!(c.outputs, codes);
    }
}
