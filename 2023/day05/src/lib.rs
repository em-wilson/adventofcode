mod shared;
mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 5: If You Give A Seed A Fertilizer".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::fetch_lowest_location(&input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::fetch_lowest_location_by_seed_ranges(&input).to_string()
}