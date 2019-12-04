use regex::Regex;
use std::collections::BTreeSet;
use std::ops::Add;

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

#[derive(Debug, PartialEq)]
enum Direction {
    Up(i32),
    Left(i32),
    Down(i32),
    Right(i32),
}

pub fn answer1() {
    let (w1, w2) = read_wires();
    println!("{}", distance(closest_intersection(w1, w2)));
}

pub fn answer2() {
    let (w1, w2) = read_wires();
    println!("{}", shortest_intersection(w1, w2));
}

fn closest_intersection(wire1: Vec<Direction>, wire2: Vec<Direction>) -> (i32, i32) {
    let p1 = walk_path(wire1);
    let p2 = walk_path(wire2);

    let s1: BTreeSet<(i32, i32)> = p1.into_iter().collect();
    let s2: BTreeSet<(i32, i32)> = p2.into_iter().collect();

    let mut all_intersections: Vec<(i32, i32)> = s1.intersection(&s2).cloned().collect();
    all_intersections.sort_by(|x, y| distance(*x).cmp(&distance(*y)));

    // skip the first element since it's (0,0)
    all_intersections[1]
}

fn shortest_intersection(wire1: Vec<Direction>, wire2: Vec<Direction>) -> usize {
    let p1 = walk_path(wire1);
    let p2 = walk_path(wire2);

    let s1: BTreeSet<(i32, i32)> = p1.clone().into_iter().collect();
    let s2: BTreeSet<(i32, i32)> = p2.clone().into_iter().collect();

    let all_intersections: Vec<(i32, i32)> = s1.intersection(&s2).cloned().collect();

    let mut step1 = 0;
    let mut step2 = 0;
    let mut min_step = 0;

    for intersection in all_intersections {
        if intersection == (0, 0) {
            continue;
        }
        for (i, p) in (*p1).into_iter().enumerate() {
            if *p == intersection {
                step1 = i;
                break;
            }
        }

        for (i, p) in (*p2).into_iter().enumerate() {
            if *p == intersection {
                step2 = i;
                break;
            }
        }

        if min_step == 0 || step1 + step2 < min_step {
            min_step = step1 + step2;
        }
    }

    min_step
}

fn walk_path(path: Vec<Direction>) -> Vec<(i32, i32)> {
    let mut s = Vec::new();
    let mut x = (0, 0);
    for dir in path {
        let (new_x, mut new_s) = walk_direction(x, dir);
        s.append(&mut new_s);
        x = new_x
    }
    s
}

fn walk_direction(start_point: (i32, i32), dir: Direction) -> ((i32, i32), Vec<(i32, i32)>) {
    let (step, n) = match dir {
        Direction::Up(n) => ((1, 0), n),
        Direction::Left(n) => ((0, -1), n),
        Direction::Down(n) => ((-1, 0), n),
        Direction::Right(n) => ((0, 1), n),
    };
    let mut v = Vec::new();
    let mut x = start_point;
    for _ in 0..n {
        v.push(x);
        x = add(x, step);
    }

    (x, v)
}

fn add<T: Add<Output = T>>(a: (T, T), b: (T, T)) -> (T, T) {
    let (ax, ay) = a;
    let (bx, by) = b;
    (ax + bx, ay + by)
}

fn distance((a, b): (i32, i32)) -> i32 {
    i32::abs(a) + i32::abs(b)
}

fn parse_path(path: &str) -> Result<Vec<Direction>, String> {
    path.split(',').map(parse_direction).collect()
}

fn parse_direction(raw: &str) -> Result<Direction, String> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^([ULDR])(\d+)$").unwrap();
    }
    let caps = RE.captures(raw).unwrap();

    let raw_num = caps
        .get(2)
        .ok_or(format!("No number matched for: {}", raw))?
        .as_str();
    let num = i32::from_str_radix(raw_num, 10).unwrap();

    match caps.get(1).unwrap().as_str() {
        "U" => Ok(Direction::Up(num)),
        "L" => Ok(Direction::Left(num)),
        "D" => Ok(Direction::Down(num)),
        "R" => Ok(Direction::Right(num)),
        d => Err(format!("Unknown direction: {}", d)),
    }
}

fn read_wires() -> (Vec<Direction>, Vec<Direction>) {
    let f = File::open("data/2019/day03.txt").unwrap();
    let fd = BufReader::new(f);
    let lines: Vec<String> = fd.lines().map(|x| x.unwrap()).collect();

    let wire1 = parse_path(&(lines[0])[..]).unwrap();
    let wire2 = parse_path(&(lines[1])[..]).unwrap();

    (wire1, wire2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_direction() {
        assert_eq!(parse_direction("U12"), Ok(Direction::Up(12)));
        assert_eq!(parse_direction("L1"), Ok(Direction::Left(1)));
        assert_eq!(parse_direction("D123"), Ok(Direction::Down(123)));
        assert_eq!(parse_direction("R42"), Ok(Direction::Right(42)));
    }

    #[test]
    fn test_parse_path() {
        assert_eq!(
            parse_path("U7,R6"),
            Ok(vec![Direction::Up(7), Direction::Right(6)])
        )
    }

    #[test]
    fn test_closest_intersection1() {
        assert_eq!(
            distance(closest_intersection(
                parse_path("R8,U5,L5,D3").unwrap(),
                parse_path("U7,R6,D4,L4").unwrap(),
            )),
            6
        );
    }

    #[test]
    fn test_closest_intersection2() {
        assert_eq!(
            distance(closest_intersection(
                parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
                parse_path("U62,R66,U55,R34,D71,R55,D58,R83").unwrap(),
            )),
            159
        );
    }

    #[test]
    fn test_closest_intersection3() {
        assert_eq!(
            distance(closest_intersection(
                parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
                parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(),
            )),
            135
        );
    }

    #[test]
    fn test_shortest_intersection1() {
        assert_eq!(
            shortest_intersection(
                parse_path("R8,U5,L5,D3").unwrap(),
                parse_path("U7,R6,D4,L4").unwrap(),
            ),
            30
        );
    }

    #[test]
    fn test_shortest_intersection2() {
        assert_eq!(
            shortest_intersection(
                parse_path("R75,D30,R83,U83,L12,D49,R71,U7,L72").unwrap(),
                parse_path("U62,R66,U55,R34,D71,R55,D58,R83").unwrap(),
            ),
            610
        );
    }

    #[test]
    fn test_shortest_intersection3() {
        assert_eq!(
            shortest_intersection(
                parse_path("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51").unwrap(),
                parse_path("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7").unwrap(),
            ),
            410
        );
    }
}
