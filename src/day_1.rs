use std::fs;

pub fn read_input() -> Vec<i32> {
    let input = fs::read_to_string("day_1_input.txt").unwrap();
    input
        .lines()
        .map(|i| i.trim().parse::<i32>().unwrap())
        .collect()
}

pub fn two_inputs_sum_2020(input: Vec<i32>) -> (i32, i32) {
    for x in &input {
        for y in &input {
            if *x + *y == 2020 {
                return (*x, *y)
            }
        }
    }
    panic!("Failed to find items that sum to 2020");
}

pub fn three_inputs_sum_2020(input: Vec<i32>) -> (i32, i32, i32) {
    for x in &input {
        for y in &input {
            for z in &input {
                if *x + *y + *z == 2020 {
                    return (*x, *y, *z)
                }
            }
        }
    }
    panic!("Failed to find items that sum to 2020");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_read_input() {
        let it = read_input();
        assert_eq!(200, it.len())
    }

    #[test]
    fn finds_2_values_that_add_to_2020() {
        let input = read_input();
        let t = two_inputs_sum_2020(input);
        assert_eq!((1359,661), t)
    }

    #[test]
    fn finds_3_values_that_add_to_2020() {
        let input = read_input();
        let t = three_inputs_sum_2020(input);
        assert_eq!((354,1369, 297), t)
    }
}
