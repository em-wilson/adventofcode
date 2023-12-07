mod solution;

pub fn run_a(input:&str) {
    // 247796702 is too low
    println!("Results for part A: {}", solution::determine_winnings(input, false));
}

pub fn run_b(input:&str) {
    println!("Results for part B: {}", solution::determine_winnings(input, true));
}
