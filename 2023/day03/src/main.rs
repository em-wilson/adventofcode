use std::fs;
// use crate::solution;
mod position;
mod solution;

use day03::{run_a, run_b};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for part A: {}", run_a(&input));
    println!("Results for part B: {}", run_b(&input));
}