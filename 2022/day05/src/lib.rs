mod solution;

use advent_shared::AdventChallenge;
use crate::solution::{figure_top_crates, TowerModel};

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 5: Supply Stacks".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(input:&str) -> String {
    figure_top_crates(TowerModel::Tower9000, input).to_string()
}

pub fn run_b(input:&str) -> String {
    figure_top_crates(TowerModel::Tower9001, input).to_string()
}
