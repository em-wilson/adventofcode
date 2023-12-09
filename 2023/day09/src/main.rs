use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for A: {}", day09::run_a(&input));

    println!("Results for B: {}", day09::run_b(&input));
}