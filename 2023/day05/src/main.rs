use std::fs;

mod shared;
mod solution;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for A: {}", solution::fetch_lowest_location(&input));
    println!("Results for B: {}", solution::fetch_lowest_location_by_seed_ranges(&input));
}