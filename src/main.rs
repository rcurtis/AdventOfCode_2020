#![allow(warnings)]

use crate::day_8::{Instruction, Token};

mod day_1;
mod day_2;
mod day_3;
mod day_4;
mod day_5;
mod util;
mod day_6;
mod day_7;
mod day_8;
mod day_9;

fn main() {
    do_day_1_part_1();
    do_day_1_part_2();

    do_day_2_part_1();
    do_day_2_part_2();

    do_day_3_part_1();
    do_day_3_part_2();

    do_day_4_part_1();
    do_day_4_part_2();

    do_day_5_part_1();
    do_day_5_part_2();

    do_day_6_part_1();
    do_day_6_part_2();

    do_day_8_part_1();
    do_day_8_part_2();

    do_day_9_part_1();
    do_day_9_part_2();
}

fn do_day_1_part_1() {
    let input = day_1::read_input();
    let (a, b) = day_1::two_inputs_sum_2020(input);
    println!("Found {} and {} that add up to 2020", &a, &b);
    println!("{} * {} = {}", &a, &b, a * b);
}


fn do_day_1_part_2() {
    let input = day_1::read_input();
    let (a, b, c) = day_1::three_inputs_sum_2020(input);
    println!("Found {}, {}, and {} that add up to 2020", &a, &b, &c);
    println!("{} * {} * {} = {}", &a, &b, &c, a * b * c);
}


fn do_day_2_part_1() {
    let input = day_2::read_input();
    let count = day_2::count_valid(&input);
    println!("Found {} valid passwords, {} invalid passwords", count, input.len() - count);
}

fn do_day_2_part_2() {
    let input = day_2::read_input();
    let count = day_2::count_valid_part_2(&input);
    println!("Found {} valid passwords, {} invalid passwords", count, input.len() - count);
}

fn do_day_3_part_1() {
    let input = day_3::read_input();
    let count = day_3::part_1(&input);
    println!("Ran into {} trees.", count);
}

fn do_day_3_part_2() {
    let input = day_3::read_input();
    let count = day_3::part_2(&input);
    println!("Ran into {} trees.", count);
}

fn do_day_4_part_1() {
    let input = day_4::read_input();
    let valid_count = day_4::count_valid_passports_part_1(&input);
    println!("Valid passport count part 1: {}", valid_count);
}

fn do_day_4_part_2() {
    let input = day_4::read_input();
    let valid_count = day_4::count_valid_passports_part_2(&input);
    println!("Valid passport count part 2: {}", valid_count);
}

fn do_day_5_part_1() {
    let input = day_5::read_input();
    let max_seat = day_5::max_seat_id(&input);
    println!("Day 5: Max seat id: {:?}", max_seat);
}

fn do_day_5_part_2() {
    let input = day_5::read_input();
    let missing_seat = day_5::get_missing_seat(&input);
    println!("Day 5: My seat: {:?}", missing_seat);
}

fn do_day_6_part_1() {
    let input = day_6::read_input();
    let yes_count = day_6::count_yeses_part_1(&input);
    println!("Day 6 part 1: Yes count: {:?}", yes_count);
}

fn do_day_6_part_2() {
    let input = day_6::read_input();
    let yes_count = day_6::count_yeses_part_2(&input);
    println!("Day 6 part 2: Yes count: {:?}", yes_count);
}

fn do_day_8_part_1() {
    let input = day_8::read_input();
    let mut parsed : Vec<Token> = input.iter().map(|x| day_8::str_to_instruction(x)).collect();
    let loop_acc = day_8::acc_value_infinite_loop(&parsed);
    println!("Accumulator when loop detected: {}", loop_acc);
}

fn do_day_8_part_2() {
    let input = day_8::read_input();
    let mut parsed : Vec<Token> = input.iter().map(|x| day_8::str_to_instruction(x)).collect();
    let loop_acc = day_8::swap_instructions_until_winner_found(&parsed);
    println!("Accumulator after finding winning combo: {}", loop_acc);
}

fn do_day_9_part_1() {
    let input = day_9::load_input();
    let actual = day_9::find_first_invalid(&input, 25);
    println!("First value that did was not sum in previous 25: {}", actual);
}

fn do_day_9_part_2() {
    let input = day_9::load_input();
    let actual = day_9::numbers_that_sum_target(&input, 1124361034);
    println!("Numbers that sum to 1124361034 -min: {}, max: {}, sum:{}",
             actual.0, actual.1, actual.0 + actual.1);
}