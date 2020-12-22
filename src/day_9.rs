use std::fs;

fn load_from_str(input: &str) -> Vec<u64> {
    input.lines()
        .map(|x| x.parse::<u64>().unwrap())
        .collect()
}

pub fn load_input() -> Vec<u64> {
    fs::read_to_string("day_9_input.txt").unwrap()
        .lines()
        .map(|x| x.parse::<u64>().expect(&format!("Invalid number: {}", x)))
        .collect()
}

pub fn find_first_invalid(input: &Vec<u64>, preamble_size: usize) -> u64 {
    let mut idx = preamble_size;

    while idx < input.len() {
        let current = input[idx];
        let search_region = &input[idx - preamble_size .. idx];

        let mut found = false;
        for (x_idx, x) in search_region.iter().enumerate() {
            for y in &search_region[x_idx ..] {
                if x + y == current {
                    found = true;
                }
            }
        }
        if !found {
            return current;
        }

        idx += 1;
    }
    panic!("Doh!")
}

pub fn numbers_that_sum_target(input: &Vec<u64>, target: u64) -> (u64, u64) {
    let mut idx = 0;

    while idx < input.len() {
        let search_range = &input[idx..];
        let mut sum: u64 = 0;
        let mut bucket = vec![];
        for i in search_range {
            sum += i;
            if sum > target {
                break;
            }

            bucket.push(*i);
            if sum == target {
                return (*bucket.iter().min().unwrap(), *bucket.iter().max().unwrap())
            }
        }
        idx += 1;
    }
    panic!("Doh!");
}

#[cfg(test)]
mod tests {
    use super::*;

    const raw_input: &str =
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part_1_test() {
        let input = load_from_str(raw_input);
        let actual = find_first_invalid(&input, 5);
        assert_eq!(127, actual);
    }

    const raw_input_part2: &str =
"35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";

    #[test]
    fn part_2_test() {
        let input = load_from_str(raw_input_part2);
        let actual = numbers_that_sum_target(&input, 127);
        assert_eq!((15, 47), actual);
    }
}