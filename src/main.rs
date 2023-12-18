use aoc_23::day_one::{faster_str_as_u32, prepare_input, sum_calibration_values};

fn main() {
    let test_input = include_str!("../res/day_one_pt_two_test.txt");
    let data_two = prepare_input(test_input, faster_str_as_u32);
    let sum = sum_calibration_values(data_two);
    println!("sum: {}", sum);
}
