mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 10: Cathode-Ray Tube".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::sum_during_cycles(&input, &vec!(20, 60, 100, 140, 180, 220)).to_string()
}

pub fn run_b(input:&str) -> String {
    String::from("\n") + solution::render_image(&input).as_str()
}
