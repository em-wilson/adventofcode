mod bookend_tokenizer;
mod number_dictionary;
mod peekable_string;
mod trebuchet;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "--- Day 1: Trebuchet?! ---".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input: &str) -> String {
    trebuchet::calibrate_input(input, true).to_string()
}

pub fn run_b(input: &str) -> String {
    trebuchet::calibrate_input(input, false).to_string()
}