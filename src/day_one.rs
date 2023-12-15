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

#[cfg(test)]

mod tests {
    use super::*;

    #[test]
    fn day_one_part_one_test() {
        let input = prepare_input(INPUT);
        assert_eq!(sum_calibration_values(input), 54708);
    }
}
