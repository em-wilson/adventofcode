use day02::game;

fn sum_possible_games(input:String) -> u32 {
    let game = game::new_game(input.to_string());
    return game.play(game::Mode::ImpossibleCombinations);
}

#[test]
fn test_sum_possible_games() {
    assert_eq!(1, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()));
    assert_eq!(3, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()));
    assert_eq!(3, sum_possible_games("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()));
}

