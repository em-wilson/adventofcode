mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 18: Lavaduct Lagoon".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_dugout_size(&input, solution::LineParser::Straight).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::count_dugout_size(&input, solution::LineParser::Hex).to_string()
}
