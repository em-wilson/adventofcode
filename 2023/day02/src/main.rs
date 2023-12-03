use std::fs;

mod game;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    let game = game::new_game(input.to_string());
    println!("Score for part A: {}", game.play(game::Mode::ImpossibleCombinations));
    println!("Score for part B: {}", game.play(game::Mode::PowerCubes));
}