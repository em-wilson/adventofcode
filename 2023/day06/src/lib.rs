mod solution;

use advent_shared::AdventChallenge;

pub fn create_challenge() -> AdventChallenge {
    AdventChallenge{
        title: "Day 6: Wait For It".to_string(),
        part_a: Box::new(|input| run_a(input)),
        part_b: Box::new(|input| run_b(input)),
    }
}

pub fn run_a(_input:&str) -> String {
    // (time, distance)
    let races = vec!(
        (58, 434),
        (81, 1041),
        (96, 2219),
        (76, 1218)
    );
    solution::multiply_possible_hold_time(races).to_string()
}

pub fn run_b(_input:&str) -> String {
    solution::multiply_possible_hold_time(vec!((58819676, 434104122191218))).to_string()
}