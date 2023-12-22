mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 22: Sand Slabs".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::dissolve_safe_bricks(input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::count_falling_bricks(input).to_string()
}