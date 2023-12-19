mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 19: Aplenty".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::sum_parts_rating(&input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::sum_parts_combinations(&input).to_string()
}
