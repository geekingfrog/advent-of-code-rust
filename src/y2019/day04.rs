const MIN_VAL: u32 = 273025;
const MAX_VAL: u32 = 767253;

pub fn answer1() {
    let stuff: Vec<u32> = (MIN_VAL..MAX_VAL + 1)
        .into_iter()
        .filter(|x| is_potential_password1(*x))
        .collect();

    println!("{}", stuff.len());
}

pub fn answer2() {
    let stuff: Vec<u32> = (MIN_VAL..MAX_VAL + 1)
        .into_iter()
        .filter(|x| is_potential_password2(*x))
        .collect();

    println!("{}", stuff.len());
}

fn is_potential_password1(number: u32) -> bool {
    has_increasing_digits(number) && has_two_consecutive_identical_digits(number)
}

fn is_potential_password2(number: u32) -> bool {
    has_increasing_digits(number)
        && not_part_larget_group(number)
}

fn has_increasing_digits(number: u32) -> bool {
    if number < 10 {
        return true;
    }

    let mut n = number / 10;
    let mut last_digit = number % 10;

    while n != 0 {
        let d = n % 10;
        if d > last_digit {
            return false;
        }
        last_digit = d;
        n = n / 10;
    }
    true
}

fn has_two_consecutive_identical_digits(number: u32) -> bool {
    if number < 10 {
        return false;
    }

    let mut n = number / 10;
    let mut last_digit = number % 10;

    while n != 0 {
        let d = n % 10;
        if last_digit == d {
            return true;
        }
        last_digit = d;
        n = n / 10;
    }
    false
}

fn not_part_larget_group(number: u32) -> bool {
    if number < 10 {
        return true;
    }

    let mut n = number / 10;
    let mut last_digit = number % 10;
    let mut seq_len = 1;

    while n != 0 {
        let d = n % 10;
        if last_digit == d {
            seq_len = seq_len + 1
        } else {
            if seq_len == 2 {
                return true;
            }
            seq_len = 1;
            last_digit = d;
        }
        n = n / 10;
    }
    seq_len == 2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_increasing_digits() {
        assert_eq!(has_increasing_digits(1), true);
        assert_eq!(has_increasing_digits(123), true);
        assert_eq!(has_increasing_digits(132), false);
        assert_eq!(has_increasing_digits(111), true);
        assert_eq!(has_increasing_digits(315999), false);
    }

    #[test]
    fn test_has_two_consecutive_identical_digits() {
        assert_eq!(has_two_consecutive_identical_digits(1), false);
        assert_eq!(has_two_consecutive_identical_digits(12), false);
        assert_eq!(has_two_consecutive_identical_digits(123123), false);
        assert_eq!(has_two_consecutive_identical_digits(11), true);
        assert_eq!(has_two_consecutive_identical_digits(12443), true);
    }

    #[test]
    fn test_is_potential_password1() {
        assert_eq!(is_potential_password1(111111), true);
        assert_eq!(is_potential_password1(223450), false);
        assert_eq!(is_potential_password1(123789), false);
    }

    #[test]
    fn test_not_part_larget_group() {
        assert_eq!(not_part_larget_group(112233), true);
        assert_eq!(not_part_larget_group(123444), false);
        assert_eq!(not_part_larget_group(111122), true);
        assert_eq!(not_part_larget_group(111123), false);
        assert_eq!(not_part_larget_group(689999), false);
        assert_eq!(not_part_larget_group(126666), false);
        assert_eq!(not_part_larget_group(116666), true);
    }
}
