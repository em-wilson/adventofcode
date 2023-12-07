mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "--- Day 7: Camel Cards ---".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::determine_winnings(input, false).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::determine_winnings(input, true).to_string()
}
