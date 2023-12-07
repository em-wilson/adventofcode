mod shared;
mod solution;

pub fn run_a(input:&str) {
    println!("Results for A: {}", solution::fetch_lowest_location(&input));
}

pub fn run_b(input:&str) {
    println!("Results for B: {}", solution::fetch_lowest_location_by_seed_ranges(&input));
}