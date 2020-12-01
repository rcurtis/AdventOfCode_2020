#![allow(warnings)]

mod day_1;

fn main() {
    do_day_1_part_1();
    do_day_1_part_2();
}

fn do_day_1_part_1() {
    let input = day_1::read_input();
    let (a,b) = day_1::two_inputs_sum_2020(input);
    println!("Found {} and {} that add up to 2020", &a, &b);
    println!("{} * {} = {}", &a, &b, a * b);
}


fn do_day_1_part_2() {
    let input = day_1::read_input();
    let (a,b,c) = day_1::three_inputs_sum_2020(input);
    println!("Found {}, {}, and {} that add up to 2020", &a, &b, &c);
    println!("{} * {} * {} = {}", &a, &b, &c, a * b * c);
}
