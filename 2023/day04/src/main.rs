use std::fs;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    let cards = solution::parse_card_input(input.to_string());
    println!("Results for part A: {}", solution::sum_card_values(&cards));
    println!("Results for part B: {}", solution::explode_and_count_cards(&cards));
}