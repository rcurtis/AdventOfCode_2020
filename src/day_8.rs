use crate::day_8::Instruction::*;
use crate::util;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Instruction {
    Jmp(i32),
    Acc(i32),
    Nop(i32),
}

#[derive(PartialEq, Debug, Clone)]
pub struct Token {
    visited: bool,
    instruction: Instruction,
}

pub fn read_input() -> Vec<String> {
    util::file_to_vec("day_8_input.txt")
}

pub fn str_to_instruction(input: &str) -> Token {
    let (instr, amnt) = input.split_at(3);
    let magnitude: i32 = amnt.trim().parse().expect(&format!("Failed to parse {}", amnt));
    let instruction = match instr {
        "jmp" => Jmp(magnitude),
        "acc" => Acc(magnitude),
        "nop" => Nop(magnitude),
        _ => panic!("Failed on input: {:?}", input)
    };
    Token { visited: false, instruction }
}

pub fn input_to_instructions(input: &str) -> Vec<Token> {
    input
        .lines()
        .map(|x| str_to_instruction(x))
        .collect()
}

pub fn acc_value_infinite_loop(instructions: &Vec<Token>) -> i32 {
    let mut idx = 0_i32;
    let mut accumulator = 0_i32;
    let mut instructions = instructions.clone();

    loop {
        let mut token = &mut instructions[idx as usize];

        if token.visited { return accumulator; }

        match token.instruction {
            Jmp(offset) => {
                idx += offset;
            }
            Acc(add) => {
                accumulator += add;
                idx += 1;
            }
            Nop(_) => {
                idx += 1;
            }
        }
        token.visited = true;
    }
}

pub fn runs_to_end(input: &Vec<Token>) -> Option<i32> {
    let mut idx = 0_i32;
    let mut accumulator = 0_i32;
    let mut instructions = input.clone();

    loop {
        if idx as usize >= input.len() {
            return Some(accumulator);
        }

        let mut token = &mut instructions[idx as usize];

        // Found a loop, this run is no good
        if token.visited {
            return None;
        }

        match token.instruction {
            Jmp(offset) => {
                idx += offset;
            }
            Acc(add) => {
                accumulator += add;
                idx += 1;
            }
            Nop(_) => {
                idx += 1;
            }
        }
        token.visited = true;
    }
}

fn swap_next_nop_or_jmp(input: &Vec<Token>, last_idx: Option<usize>) -> (Vec<Token>, usize) {
    let mut idx = if let None = last_idx { 0 } else { last_idx.unwrap() + 1 };

    let mut input = input.clone();
    for token in &mut input[idx..] {
        match token.instruction {
            Jmp(i) => {token.instruction = Nop(i); break;},
            Nop(i) => {token.instruction = Jmp(i); break;},
            _ => ()
        }
        idx += 1;
    }
    (input, idx)
}

pub fn swap_instructions_until_winner_found(input: &Vec<Token>) -> i32 {
    let mut mod_idx = None;

    loop {
        let (mut input, new_idx) = swap_next_nop_or_jmp(&input, mod_idx);
        mod_idx = Some(new_idx);
        if let Some(total) = runs_to_end(&input) {
            return total;
        }
    }

    panic!("Doh!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_to_instr1() {
        let expected = Token { visited: false, instruction: Jmp(123) };
        let actual = str_to_instruction("jmp 123");
        assert_eq!(expected, actual);
    }

    #[test]
    fn str_to_instr2() {
        let expected = Token { visited: false, instruction: Acc(123) };
        let actual = str_to_instruction("acc 123");
        assert_eq!(expected, actual);
    }

    #[test]
    fn str_to_instr3() {
        let expected = Token { visited: false, instruction: Nop(123) };
        let actual = str_to_instruction("nop 123");
        assert_eq!(expected, actual);
    }

    #[test]
    fn str_to_instr4() {
        let expected = Token { visited: false, instruction: Jmp(-123) };
        let actual = str_to_instruction("jmp -123");
        assert_eq!(expected, actual);
    }

    #[test]
    fn parse_all_input() {
        let input = read_input();
        let result = input.iter().map(|x| str_to_instruction(x));
        assert!(true);
    }

    const swap_input: &str =
        "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";

    #[test]
    fn input_to_tokens() {
        let tokens = input_to_instructions(swap_input)
            .iter()
            .map(|x| x.instruction)
            .collect::<Vec<_>>();
        let expected = vec![Nop(0), Acc(1), Jmp(4), Acc(3), Jmp(-3), Acc(-99),
                            Acc(1), Jmp(-4), Acc(6)];
        assert_eq!(expected, tokens);
    }

    #[test]
    fn swap_does_things_hjgfu09808yasd980() {
        let tokens = input_to_instructions(swap_input);
        let i = swap_instructions_until_winner_found(&tokens);
        assert_eq!(8, i);
    }
}