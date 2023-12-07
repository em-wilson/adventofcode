use std::fs;
// use crate::solution;
mod position;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Sum for part A: {}", solution::calculate_value_of_number_with_adjacent_symbols(input.to_string()));
    println!("Sum for part B: {}", solution::calculate_value_of_gears(input.to_string()));
}