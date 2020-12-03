#![allow(warnings)]

mod day_1;
mod day_2;
mod day_3;

fn main() {
    do_day_1_part_1();
    do_day_1_part_2();

    do_day_2_part_1();
    do_day_2_part_2();

    do_day_3_part_1();
    do_day_3_part_2();
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



