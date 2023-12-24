mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 24: Never Tell Me The Odds".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    // 302 is too low
    solution::count_intersections(&input, 200000000000000_f64, 400000000000000_f64).to_string()
}

pub fn run_b(_input:&str) -> String {
    String::from("---")
}
