use aoc_23::day_one::{prepare, prepare_slow, sum_calibration_values, INPUT};

fn main() {
    let test = "1abctwo3four";
    let split_numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let test_input = include_str!("../res/day_one_pt_two_test.txt");
    let data = prepare(test);
    let data_two = prepare_slow(test);
    let sum = sum_calibration_values(data);
    println!("sum: {}", sum);
}
