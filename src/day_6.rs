use std::fs;
use std::collections::HashSet;

/// Chunks are seperated by empty lines
fn split_to_chunks(input: &str) -> Vec<Vec<String>> {
    let s : Vec<&str> = input.split("\r\n\r").collect();
    let mut ret = vec![];

    for chunk in s {
        let split : Vec<String> = chunk.split("\n").map(|x| x.to_string()).collect();
        ret.push(split);
    }
    ret
}

pub fn read_input() -> Vec<Vec<String>> {
    let raw = fs::read_to_string("day_6_input.txt").unwrap();
    split_to_chunks(&raw)
}

pub fn count_yeses(input: &Vec<Vec<String>>) -> usize {
    let mut total_count = 0_usize;
    for group in input {
        let mut set = HashSet::new();
        let chs : Vec<char> = group.iter()
            .flat_map(|row| row.chars())
            .filter(|&x| x != '\n' && x != '\r')
            .collect();
        for ch in chs {
            set.insert(ch);
        }
        total_count += set.len();
    }

    total_count
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "abc\r\n\ra\r\nb\r\nc\r\n\rab\r\nac\r\n\r\n\ra\r\na\r\na\r\na\r\n\r\n\rb";

    #[test]
    fn step_1() {
        let chunks = split_to_chunks(TEST_INPUT);
        let yes_count = count_yeses(&chunks);
        assert_eq!(11, yes_count);
    }

    #[test]
    fn test_with_actual_input() {
        let input = read_input();
        let i = count_yeses(&input);
        assert_eq!(6430, i);
    }
}