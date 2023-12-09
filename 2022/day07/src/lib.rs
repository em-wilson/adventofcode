mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 7: No Space Left On Device".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    let fsi = solution::FileSystemInterface::parse(input.to_string());
    return solution::sum_filesizes_under_100000(&fsi).to_string();
}

pub fn run_b(input:&str) -> String {
    // 12534602 is too high
    let fsi = solution::FileSystemInterface::parse(input.to_string());
    return solution::lowest_to_free(&fsi, 70000000, 30000000).to_string();
}
