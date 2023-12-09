mod snafu;
mod solution;

use advent_shared::AdventChallenge;
use crate::solution::sum_fuel_inputs;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 25: Full of Hot Air".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    snafu::from_number(sum_fuel_inputs(input)).to_string()
}

pub fn run_b(_input:&str) -> String {
    "---".to_string()
}
