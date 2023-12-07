mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "--- Day 4: Scratchcards ---".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    let cards = solution::parse_card_input(input.to_string());
    solution::sum_card_values(&cards).to_string()
}

pub fn run_b(input:&str) -> String {
    let cards = solution::parse_card_input(input.to_string());
    solution::explode_and_count_cards(&cards).to_string()
}