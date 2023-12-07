use std::fs;

use day07::{run_a, run_b};

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("--- Day 7: Camel Cards ---");
    run_a(&input);
    run_b(&input);
}