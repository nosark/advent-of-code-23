use aoc_23::day_one::{prepare_input, sum_calibration_values, INPUT};
use std::collections::HashSet;
fn main() {
    let test = "1abctwo3four";
    let split_numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let test_input = include_str!("../res/day_one_pt_two_test.txt");
    let data = prepare(INPUT);
    let sum = sum_nums(data);
    println!("sum: {}", sum);
}

fn convert_vec_to_num(vec: Vec<String>) -> u32 {
    let mut num: String = "".to_string();
    if vec.len() == 1 {
        num.push_str(&vec[0]);
        num.push_str(&vec[0]);
    } else {
        num.push_str(&vec[0]);
        num.push_str(&vec[vec.len() - 1]);
    }
    num.parse::<u32>().unwrap()
}
fn prepare(input: &str) -> Vec<u32> {
    input.lines().map(|line| str_as_u32(line)).collect()
}

fn sum_nums(nums: Vec<u32>) -> u32 {
    nums.iter().sum()
}

fn string_as_u32(string: &str) -> &str {
    match string {
        "one" => "1",
        "two" => "2",
        "three" => "3",
        "four" => "4",
        "five" => "5",
        "six" => "6",
        "seven" => "7",
        "eight" => "8",
        "nine" => "9",
        _ => string,
    }
}

fn str_as_u32(line: &str) -> u32 {
    let split_numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let words: HashSet<&str> = HashSet::from_iter(split_numbers.iter().cloned());
    let mut numbers = Vec::new();
    for i in 0..line.len() {
        for j in i..line.len() + 1 {
            let word = &line[i..j];
            let word = word.chars().collect::<String>();
            if words.contains(word.as_str()) {
                let new_word = string_as_u32(word.as_str()).to_string();
                numbers.push(new_word);
            }
        }
    }

    let num = convert_vec_to_num(numbers);
    num
}
