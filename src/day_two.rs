pub const INPUT: &str = include_str!("../res/day_two.txt");
pub const TEST_INPUT: &str = include_str!("../res/day_two_p1_sample.txt");

#[derive(Copy, Clone, Debug)]
pub struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
    total_cubes: u32,
}

pub fn prepare_input(input: &str) -> Vec<Game> {
    let result = input
        .lines()
        .map(|line| {
            line.split(|c| c == ':' || c == ';' || c == ',' || c == ' ')
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let result = result
        .iter()
        .map(|line| {
            line.iter()
                .filter(|s| !s.is_empty())
                .map(|s| *s)
                .collect::<Vec<&str>>()
        })
        .collect::<Vec<Vec<&str>>>();

    let games = convert_lines_to_games(result);
    games
}

fn convert_lines_to_games(lines: Vec<Vec<&str>>) -> Vec<Game> {
    let mut games: Vec<Game> = Vec::new();
    let mut num_cubes = 0;
    let mut is_valid = true;
    lines.iter().for_each(|line| {
        let mut game = Game {
            id: 0,
            red: 0,
            green: 0,
            blue: 0,
            total_cubes: 0,
        };

        for (i, s) in line.iter().enumerate() {
            match i {
                1 => game.id = s.parse::<u32>().unwrap(),
                _ => {
                    if i % 2 == 0 && i != 0 {
                        num_cubes = s.parse::<u32>().unwrap_or_else({
                            |_| panic!("Failed to parse number of cubes: {}", s)
                        });
                    }
                    match s {
                        &"red" => {
                            is_valid = if num_cubes > 12 { false } else { true };
                            game.red += num_cubes;
                        }
                        &"green" => {
                            is_valid = if num_cubes > 13 { false } else { true };
                            game.green += num_cubes;
                        }
                        &"blue" => {
                            is_valid = if num_cubes > 14 { false } else { true };
                            game.blue += num_cubes;
                        }
                        _ => (),
                    }
                }
            }
            game.total_cubes = game.red + game.green + game.blue;
            if !is_valid {
                break;
            }
        }

        if is_valid {
            games.push(game);
        }

        is_valid = true;
    });

    games
}

pub fn sum_ids(games: &Vec<Game>) -> u32 {
    games.iter().fold(0, |acc, game| acc + game.id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_day_two_sample() {
        let input = prepare_input(TEST_INPUT);
        let result = sum_ids(&input);
        assert_eq!(result, 8);
    }

    #[test]
    fn test_day_two_part_one() {
        let input = prepare_input(INPUT);
        let result = sum_ids(&input);
        assert_eq!(result, 2348);
    }
}
