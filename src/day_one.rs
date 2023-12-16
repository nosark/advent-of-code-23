use std::collections::HashSet;
pub const INPUT: &str = include_str!("../res/day_one.txt");

pub fn prepare_input(input: &str) -> Vec<u32> {
    input
        .lines()
        .map(create_u32_from_line)
        .collect::<Vec<u32>>()
}

pub fn create_u32_from_line(line: &str) -> u32 {
    let mut nums = Vec::<char>::new();
    for c in line.chars() {
        if c.is_numeric() {
            nums.push(c);
        }
    }

    let mut str_to_conv = String::new();
    if nums.len() == 1 {
        str_to_conv.push(nums[0]);
        str_to_conv.push(nums[0]);
    } else {
        str_to_conv.push(nums[0]);
        str_to_conv.push(nums[nums.len() - 1]);
    }

    str_to_conv.parse::<u32>().unwrap()
}

pub fn sum_calibration_values(input: Vec<u32>) -> u32 {
    input.iter().sum()
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

pub fn prepare_slow(input: &str) -> Vec<u32> {
    input.lines().map(|line| str_as_u32(line)).collect()
}

pub fn prepare(input: &str) -> Vec<u32> {
    input.lines().map(|line| faster_str_as_u32(line)).collect()
}

fn string_as_numeric(string: &str) -> &str {
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
                let new_word = string_as_numeric(word.as_str()).to_string();
                numbers.push(new_word);
            }
        }
    }

    let num = convert_vec_to_num(numbers);
    num
}

fn faster_str_as_u32(line: &str) -> u32 {
    let split_numbers = [
        "1", "2", "3", "4", "5", "6", "7", "8", "9", "one", "two", "three", "four", "five", "six",
        "seven", "eight", "nine",
    ];
    let words: HashSet<&str> = HashSet::from_iter(split_numbers.iter().cloned());
    let mut numbers = Vec::new();
    let mut i = 0;
    let mut j = i + 1;
    while i < line.len() {
        while j <= line.len() {
            let word = &line[i..j];
            let word = word.chars().collect::<String>();
            if words.contains(word.as_str()) {
                let new_word = string_as_numeric(word.as_str()).to_string();
                numbers.push(new_word);
                i = j - 1;
                break;
            } else {
                j += 1;
            }
        }
        i += 1;
        j = i + 1;
    }

    let num = convert_vec_to_num(numbers);
    num
}

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn day_one_part_one_test() {
        let input = prepare_input(INPUT);
        assert_eq!(sum_calibration_values(input), 54708);
    }

    #[test]
    fn day_one_part_two_test() {
        let input = prepare(INPUT);
        assert_eq!(sum_calibration_values(input), 54087);
    }
    #[test]
    fn day_one_part_two_slow_test() {
        let input = prepare_slow(INPUT);
        assert_eq!(sum_calibration_values(input), 54087);
    }
}
