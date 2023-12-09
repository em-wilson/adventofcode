use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    // 1362044 -- too high
    println!("Results for A: {}", day07::run_a(&input));

    println!("Results for B: {}", day07::run_b(&input));
}