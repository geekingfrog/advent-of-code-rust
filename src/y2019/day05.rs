use y2019::computer as computer;

pub fn answer1() {
    let codes = computer::read_codes("data/2019/day05.txt");
    let mut computer = computer::Computer::new(codes);
    computer.run_with_inputs(vec![1]).unwrap();
    println!("{:?}", computer.outputs.last().unwrap());
}

pub fn answer2() {
    let codes = computer::read_codes("data/2019/day05.txt");
    let mut computer = computer::Computer::new(codes);
    computer.run_with_inputs(vec![5]).unwrap();
    println!("{:?}", computer.outputs.last().unwrap());
}
