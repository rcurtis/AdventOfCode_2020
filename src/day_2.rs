use std::fs;

pub struct Pass {
    ch: char,
    min: usize,
    max: usize,
    password: String
}

pub fn read_input() -> Vec<Pass> {
    let input = fs::read_to_string("day_2_input.txt").unwrap();
    input
        .lines()
        .map(parse_rule_and_password)
        .collect()
}

pub fn is_valid_password(pass: &Pass) -> bool {
    let ch_count = pass.password
        .chars()
        .filter(|x| *x == pass.ch)
        .count();
    ch_count >= pass.min && ch_count <= pass.max
}

pub fn count_valid(input: &Vec<Pass>) -> usize {
    input.iter()
        .filter(|x| is_valid_password(*x))
        .count()
}

pub fn is_valid_password_part_2(pass: &Pass) -> bool {
    let first = pass.password.chars().nth(pass.min-1).unwrap();
    let second = pass.password.chars().nth(pass.max-1).unwrap();
    (first == pass.ch && second != pass.ch) || (first != pass.ch && second == pass.ch)
}

pub fn count_valid_part_2(input: &Vec<Pass>) -> usize {
    input.iter()
        .filter(|x| is_valid_password_part_2(*x))
        .count()
}

pub fn parse_rule_and_password(input: &str) -> Pass {
    let split : Vec<_> = input.split(":").collect();
    let left = split[0];
    let password = split[1].trim();

    let left_split = left.split_ascii_whitespace().collect::<Vec<_>>();
    let ch = left_split[1].chars().next().unwrap();

    let range = left_split[0].split("-").collect::<Vec<_>>();
    let min : usize = range[0].parse().unwrap();
    let max : usize = range[1].parse().unwrap();

    Pass {
        ch,
        min,
        max,
        password: password.to_string()
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_parse() {
        let it = parse_rule_and_password("1-3 a: abcde");
        assert_eq!('a', it.ch);
        assert_eq!(1, it.min);
        assert_eq!(3, it.max);
        assert_eq!("abcde", it.password);
    }

    #[test]
    fn passes_valid1() {
        let it = parse_rule_and_password("1-3 a: abcde");
        assert!(is_valid_password(&it))
    }

    #[test]
    fn passes_valid2() {
        let it = parse_rule_and_password("2-9 c: ccccccccc");
        assert!(is_valid_password(&it))
    }

    #[test]
    fn fails_invalid1() {
        let it = parse_rule_and_password("1-3 b: cdefg");
        assert!(!is_valid_password(&it))
    }

    #[test]
    fn fails_invalid2() {
        let it = parse_rule_and_password("2-3 b: cbdefg");
        assert!(!is_valid_password(&it))
    }

    #[test]
    fn passes_valid_part_2_1() {
        let it = parse_rule_and_password("1-3 a: abcde");
        assert!(is_valid_password_part_2(&it))
    }

    #[test]
    fn passes_invalid_part_2_2() {
        let it = parse_rule_and_password("1-3 b: cdefg");
        assert!(!is_valid_password_part_2(&it))
    }

    #[test]
    fn passes_invalid_part_2_3() {
        let it = parse_rule_and_password("2-9 c: ccccccccc");
        assert!(!is_valid_password_part_2(&it))
    }
}