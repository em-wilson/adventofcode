fn highest_of(a: Option<u32>, b:u32) -> u32 {
    if a == None {
        return b;
    }

    return std::cmp::max(a.unwrap(), b);
}

pub fn calculate_game_score(game_line: String) -> u32 {
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

#[cfg(test)]
mod test {
    use super::{calculate_game_score};

    #[test]
    fn test_powercube_game_score() {
        assert_eq!(48, calculate_game_score("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
        assert_eq!(12, calculate_game_score("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
        assert_eq!(1560, calculate_game_score("Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
        assert_eq!(630, calculate_game_score("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()));
        assert_eq!(36, calculate_game_score("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()));
    }
}