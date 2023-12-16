use aoc_23::day_one::{
    faster_str_as_u32, prepare_input, str_as_u32, sum_calibration_values, INPUT,
};

fn main() {
    let test = "1abctwo3four";
    let split_numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let test_input = include_str!("../res/day_one_pt_two_test.txt");
    let data = prepare_input(test, str_as_u32);
    let data_two = prepare_input(test, faster_str_as_u32);
    let sum = sum_calibration_values(data);
    println!("sum: {}", sum);
}
