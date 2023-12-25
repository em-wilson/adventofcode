mod jerk;
// mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 24: Never Tell Me The Odds".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    String::from("12343")
}

pub fn run_b(input:&str) -> String {
    jerk::part2(&input);
    String::from("")
}
