pub mod game;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 2: Cube Conundrum".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

fn run_a(input:&str) -> String {
    let game = game::new_game(input.to_string());
    game.play(game::Mode::ImpossibleCombinations).to_string()
}

fn run_b(input:&str) -> String {
    let game = game::new_game(input.to_string());
    game.play(game::Mode::PowerCubes).to_string()
}