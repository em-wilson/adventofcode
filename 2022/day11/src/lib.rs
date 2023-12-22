mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 11: Monkey in the Middle".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(_input:&str) -> String {
    String::from("---")
}

pub fn run_b(_input:&str) -> String {
    String::from("---")
}
