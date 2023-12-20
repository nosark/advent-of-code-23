use aoc_23::day_two::{prepare_input, sum_ids, INPUT};

fn main() {
    let input = prepare_input(INPUT);
    // println!("Input: {:?}", input);
    let result = sum_ids(&input);
    println!("Input: {:?}", result);
    // println!("Input: {:?}", input[0]);
}
