mod solution;

use crate::solution::OverlapType;
use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 4: Camp Cleanup".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_overlaps(input, OverlapType::Completely).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::count_overlaps(input, OverlapType::Partial).to_string()
}