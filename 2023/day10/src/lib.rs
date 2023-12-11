mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 10: Pipe Maze".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_furthest_step(&input).to_string()
}

pub fn run_b(input:&str) -> String {
    // 6165 is too high
    solution::count_enclosed_tiles(&input).to_string()
}
