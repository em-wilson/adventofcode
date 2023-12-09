mod game;
mod play_builder;
mod solution;

use advent_shared::AdventChallenge;
use crate::play_builder::{PlayMapper, OutcomeMapper};

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 2: Rock Paper Scissors".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    return solution::game_outcome(&input, &PlayMapper{}).to_string();
}

pub fn run_b(input:&str) -> String {
    return solution::game_outcome(&input, &OutcomeMapper{}).to_string();
}
