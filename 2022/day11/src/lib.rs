mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 11: Monkey in the Middle".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    solution::count_monkey_business(&input, 20, true).to_string()
}

pub fn run_b(input:&str) -> String {
    solution::count_monkey_business(&input, 10000, false).to_string()
}
