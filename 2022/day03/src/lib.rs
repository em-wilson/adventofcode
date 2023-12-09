mod badge;
mod letter;
mod priority;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 3: Rucksack Reorganization".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    priority::sum_priority(input).to_string()
}

pub fn run_b(input:&str) -> String {
    badge::sum_priority(input).to_string()
}
