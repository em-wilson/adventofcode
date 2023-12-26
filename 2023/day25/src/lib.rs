mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 25: Snowverload".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    use std::fs;
    // rcn, xmv,
    let _ = fs::write("graph.dot", solution::output_graph(input));
    solution::find_bridges_product(input).to_string()
}

pub fn run_b(_input:&str) -> String {
    String::from("---")
}
