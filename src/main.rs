use aoc_23::day_one::{prepare_input, sum_calibration_values, INPUT};

fn main() {
    let calibration_values = prepare_input(INPUT);
    let sum = sum_calibration_values(calibration_values);
    println!("Sum of calibration values: {}", sum);
}
