mod position;
mod solution;

pub fn run_a(input:&str) {
    println!("Results for part A: {}", solution::calculate_value_of_number_with_adjacent_symbols(input.to_string()));
}

pub fn run_b(input:&str) {
    println!("Results for part B: {}", solution::calculate_value_of_gears(input.to_string()));
}