mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 8: Treetop Tree House".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_visible_trees(input).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::calculate_max_scenic_score(input).to_string()
}
