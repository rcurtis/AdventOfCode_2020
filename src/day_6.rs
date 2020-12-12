use std::fs;
use std::collections::{HashSet, HashMap};

/// Chunks are seperated by empty lines
fn split_to_chunks(input: &str) -> Vec<Vec<String>> {
    let s : Vec<&str> = input.split("\r\n\r").collect();
    let mut ret = vec![];

    for chunk in s {
        let split : Vec<String> = chunk
            .split("\n")
            .map(|x| x.to_string().replace("\r", "").replace(" ", "").replace("\n", ""))
            .filter(|x| !x.is_empty())
            .collect();
        ret.push(split);
    }
    ret
}

pub fn read_input() -> Vec<Vec<String>> {
    let raw = fs::read_to_string("day_6_input.txt").unwrap();
    split_to_chunks(&raw)
}

pub fn count_yeses_part_1(input: &Vec<Vec<String>>) -> usize {
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

pub fn count_yeses_part_2(input: &Vec<Vec<String>>) -> usize {
    let mut total_count = 0_usize;
    for group in input {
        let person_count = group.len();
        let mut answers = HashMap::new();

       for person in group {
            for answer in person.chars() {
                let entry = answers.entry(answer).or_insert(0_usize);
                *entry += 1;
            }
        }

        let agreed_yes = answers.iter()
            .filter(|(&k,&v)| v == person_count)
            .count();
        total_count += agreed_yes;
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
        let yes_count = count_yeses_part_1(&chunks);
        assert_eq!(11, yes_count);
    }

    #[test]
    fn test_with_actual_input() {
        let input = read_input();
        let i = count_yeses_part_1(&input);
        assert_eq!(6430, i);
    }

    #[test]
    fn part_2_1() {
        let chunks = split_to_chunks(TEST_INPUT);
        let yes_count = count_yeses_part_2(&chunks);
        assert_eq!(6, yes_count);
    }
}