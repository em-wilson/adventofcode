pub mod snafu;

use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");


    println!("Results for part A: {}", snafu::from_number(sum_fuel_inputs(input.to_string())));
}

fn sum_fuel_inputs(input:String) -> i64 {
    return input.split("\n")
        .map(|s| snafu::to_number(s.to_string()))
        .collect::<Vec<i64>>()
        .iter()
        .sum();
}
