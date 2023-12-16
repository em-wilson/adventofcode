mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 16: The Floor Will Be Lava".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_energized_tiles(&input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::par_count_max_energized_tiles(&input).to_string()
}
