use std::fs;

mod shared;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for part A: {}", day05::run_a(&input));
    println!("Results for part B: {}", day05::run_b(&input));
}