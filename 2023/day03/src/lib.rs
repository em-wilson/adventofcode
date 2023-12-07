mod position;
mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "--- Day 3: Gear Ratios ---".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::calculate_value_of_number_with_adjacent_symbols(input.to_string()).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::calculate_value_of_gears(input.to_string()).to_string()
}