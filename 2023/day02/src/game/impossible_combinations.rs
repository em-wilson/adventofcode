pub fn calculate_game_score(game_line: String) -> u32 {
    let game_parts: Vec<_> = game_line.split(": ").collect();
    let game_number = get_game_number_from_title(game_parts[0].to_string());
    let game_draws = game_parts[1].to_string();
    let is_possible = sum_game_scores(game_draws);
    if is_possible {
        return game_number;
    }
    return 0;
}

fn get_game_number_from_title(title: String) -> u32 {
    let title_parts: Vec<_> = title.split(" ").collect();
    return title_parts[1].to_string().parse::<u32>().unwrap();
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

#[cfg(test)]
mod test {
    use super::{sum_game_scores};

    #[test]
    fn test_sum_game_scores() {
        assert_eq!(true, sum_game_scores("3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
        assert_eq!(true, sum_game_scores("1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
        assert_eq!(false, sum_game_scores("8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
        assert_eq!(false, sum_game_scores("1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()));
        assert_eq!(true, sum_game_scores("6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()));
    }
}