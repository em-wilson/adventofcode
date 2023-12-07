use std::fs;

use day01::{run_a, run_b};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("--- Day 1: Trebuchet?! ---");
    run_a(&input);
    run_b(&input);
}