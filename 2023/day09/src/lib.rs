mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 9: Mirage Maintenance".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::sum_of_next_sequences(&input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::sum_of_prev_sequences(&input).to_string()
}
