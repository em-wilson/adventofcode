mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 21: Step Counter".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_garden_plots(&input, 64).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::count_infinite_garden_plots(&input, 26501365).to_string()
}
