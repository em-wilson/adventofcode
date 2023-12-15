mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 14: Parabolic Reflector Dish".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::calculate_load(input, 0).to_string()
}

pub fn run_b(input:&str) -> String {
    // 93707 is wrong
    // 98297 is too high
    // 98510 is too high
    // 98624 is too high
    // 136804 is too high
    solution::calculate_load(input, 1000000000).to_string()
}
