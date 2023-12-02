use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

        println!("Score for part A: {}", sum_games(input.to_string(), GameType::ImpossibleCombinations));
    
        println!("Score for part B: {}", sum_games(input.to_string(), GameType::PowerCubes));
    }

enum GameType {
    ImpossibleCombinations,
    PowerCubes
}

impl GameType {
    fn calculate(&self, input:String) -> u32 {
        return match self {
            GameType::ImpossibleCombinations => calculate_is_possible_game_score(input.to_string()),
            GameType::PowerCubes => calculate_powercube_game_score(input.to_string()),
        }
    }
}

fn sum_game_scores(input: String) -> bool {
    for group in input.split("; ") {
        for colour_group in group.split(", ") {
            let colour_info: Vec<_> = colour_group.split(" ").collect();
            let colour_count = colour_info[0].to_string().parse::<i32>().unwrap();
            let colour_name = colour_info[1];

            let is_possible = match colour_name.to_string().as_str() {
                "red" => colour_count <= 12,
                "blue" => colour_count <= 14,
                "green" => colour_count <= 13,
                _ => true
            };

            if !is_possible {
                return false;
            }
        }
    }
    return true;
}

fn get_game_number_from_title(title: String) -> u32 {
    let title_parts: Vec<_> = title.split(" ").collect();
    return title_parts[1].to_string().parse::<u32>().unwrap();
}

fn sum_games(input: String, game_mode:GameType) -> u32 {
    let mut possible_game_sum = 0;
    for game_line in input.split("\n") {
        possible_game_sum += game_mode.calculate(game_line.to_string());
    }
    return possible_game_sum;
}

fn calculate_is_possible_game_score(game_line: String) -> u32 {
    let game_parts: Vec<_> = game_line.split(": ").collect();
    let game_number = get_game_number_from_title(game_parts[0].to_string());
    let game_draws = game_parts[1].to_string();
    let is_possible = sum_game_scores(game_draws);
    if is_possible {
        return game_number;
    }
    return 0;
}

fn highest_of(a: Option<u32>, b:u32) -> u32 {
    if a == None {
        return b;
    }

    return std::cmp::max(a.unwrap(), b);
}

fn calculate_powercube_game_score(game_line: String) -> u32 {
    let mut lowest_red: Option<u32> = None;
    let mut lowest_blue: Option<u32> = None;
    let mut lowest_green: Option<u32> = None;
    let cube_line = game_line.split(": ").collect::<Vec<_>>()[1].to_string();
    for group in cube_line.split("; ") {
        for colour_group in group.split(", ") {
            let colour_info: Vec<_> = colour_group.split(" ").collect();
            let colour_count = colour_info[0].to_string().parse::<u32>().unwrap();
            let colour_name = colour_info[1];

            match colour_name.to_string().as_str() {
                "red" => lowest_red = Some(highest_of(lowest_red, colour_count)),
                "blue" => lowest_blue = Some(highest_of(lowest_blue, colour_count)),
                "green" => lowest_green = Some(highest_of(lowest_green, colour_count)),
                &_ => todo!(),
            };
        }
    }
    return lowest_red.unwrap() * lowest_blue.unwrap() * lowest_green.unwrap();
}

// 12 red cubes, 13 green cubes, and 14 blue cubes

/*
Y Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Y Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
N Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
N Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Y Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
*/
#[cfg(test)]
mod tests {
    use super::{calculate_powercube_game_score, sum_game_scores, sum_games, GameType};

    fn sum_possible_games(input:String) -> u32 {
        return sum_games(input.to_string(), GameType::ImpossibleCombinations);
    }

    fn sum_powercube_games(input:String) -> u32 {
        return sum_games(input.to_string(), GameType::PowerCubes);
    }

    #[test]
    fn test_sum_possible_games() {
        assert_eq!(1, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
        assert_eq!(3, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
        assert_eq!(3, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
    }

    #[test]
    fn test_sum_powercube_game() {
        assert_eq!(2286, sum_powercube_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()));
    }

    #[test]
    fn test_powercube_game_score() {
        assert_eq!(48, calculate_powercube_game_score("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
        assert_eq!(12, calculate_powercube_game_score("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
        assert_eq!(1560, calculate_powercube_game_score("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
        assert_eq!(630, calculate_powercube_game_score("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()));
        assert_eq!(36, calculate_powercube_game_score("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()));
    }

    #[test]
    fn test_sum_game_scores() {
        assert_eq!(true, sum_game_scores("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
        assert_eq!(true, sum_game_scores("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
        assert_eq!(false, sum_game_scores("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
        assert_eq!(false, sum_game_scores("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()));
        assert_eq!(true, sum_game_scores("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()));
    }
}