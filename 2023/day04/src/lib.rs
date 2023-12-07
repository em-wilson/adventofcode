mod solution;

pub fn run_a(input:&str) {
    let cards = solution::parse_card_input(input.to_string());
    println!("Results for part A: {}", solution::sum_card_values(&cards));
}

pub fn run_b(input:&str) {
    let cards = solution::parse_card_input(input.to_string());
    println!("Results for part B: {}", solution::explode_and_count_cards(&cards));
}