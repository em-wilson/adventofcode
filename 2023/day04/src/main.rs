use std::fs;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    let cards = solution::parse_card_input(input.to_string());
    println!("Results for part A: {}", day04::run_a(&input));
    println!("Results for part B: {}", day04::run_b(&input));
}