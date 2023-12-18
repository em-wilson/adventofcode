mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 17: Clumsy Crucible".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::calculate_shortest_path(&input, solution::CrucibleType::Regular).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::calculate_shortest_path(&input, solution::CrucibleType::Ultra).to_string()
}
