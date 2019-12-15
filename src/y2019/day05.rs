use y2019::computer as computer;

pub fn answer1() {
    println!("{:?}", solve(1));
}

pub fn answer2() {
    println!("{:?}", solve(5));
}

fn solve(input: i64) -> i64 {
    let codes = computer::read_codes("data/2019/day05.txt");
    let mut computer = computer::Computer::new(codes);
    computer.with_input(vec![input]);
    computer.run_until_halt().unwrap();
    *computer.outputs.last().unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_computer_day05() {
        assert_eq!(solve(1), 10987514);
        assert_eq!(solve(5), 14195011);
    }
}
