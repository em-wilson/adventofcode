mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 6: Tuning Trouble".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::calibrate(input, 4).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::calibrate(input, 14).to_string()
}
