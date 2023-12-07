pub mod game;

pub fn run_a(input:&str) {
    let game = game::new_game(input.to_string());
    println!("Results for part A: {}", game.play(game::Mode::ImpossibleCombinations));
}

pub fn run_b(input:&str) {
    let game = game::new_game(input.to_string());
    println!("Results for part B: {}", game.play(game::Mode::PowerCubes));
}