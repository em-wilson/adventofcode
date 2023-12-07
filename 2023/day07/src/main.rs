use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("--- Day 7: Camel Cards ---");
    println!("Results for part A: {}", day07::run_a(&input));
    println!("Results for part B: {}", day07::run_b(&input));
}