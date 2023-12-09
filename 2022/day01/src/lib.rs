mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 1: Calorie Counting".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    return solution::most_calories(&input).to_string();
}

pub fn run_b(input:&str) -> String {
    return solution::combined_most_calories(&input).to_string();
}
