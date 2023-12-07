use std::fs;

fn main() {
    run_day01();
}

fn run_day01() {
    let input = fs::read_to_string("day01/input.txt")
        .expect("Could not read file input");

    println!("--- Day 1: Trebuchet?! ---");
    day01::run_a(&input);
    day01::run_b(&input);
}
