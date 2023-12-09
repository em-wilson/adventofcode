use std::fs;

pub mod letter;
pub mod badge;
pub mod priority;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Result for part A: {}", priority::sum_priority(input.to_string()));
    println!("Result for part B: {}", badge::sum_priority(input.to_string()));
}