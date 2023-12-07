use std::fs;

fn main() {
    run_day01();
    run_day02();
    run_day03();
    run_day04();
    run_day05();
    run_day06();
    run_day07();
}

fn run_day01() {
    let input = fs::read_to_string("day01/input.txt")
        .expect("Could not read file input");

    println!("--- Day 1: Trebuchet?! ---");
    day01::run_a(&input);
    day01::run_b(&input);
}

fn run_day02() {
    let input = fs::read_to_string("day02/input.txt")
        .expect("Could not read file input");

    println!("\n--- Day 2: Cube Conundrum ---");
    day02::run_a(&input);
    day02::run_b(&input);
}

fn run_day03() {
    let input = fs::read_to_string("day03/input.txt")
        .expect("Could not read file input");

    println!("\n--- Day 3: Gear Ratios ---");
    day03::run_a(&input);
    day03::run_b(&input);
}

fn run_day04() {
    let input = fs::read_to_string("day04/input.txt")
        .expect("Could not read file input");

    println!("\n--- Day 4: Scratchcards ---");
    day04::run_a(&input);
    day04::run_b(&input);
}

fn run_day05() {
    let input = fs::read_to_string("day05/input.txt")
        .expect("Could not read file input");

    println!("\n--- Day 5: If You Give A Seed A Fertilizer ---");
    day05::run_a(&input);
    day05::run_b(&input);
}

fn run_day06() {
    println!("\n--- Day 6: Wait For It ---");
    day06::run_a("");
    day06::run_b("");
}

fn run_day07() {
    let input = fs::read_to_string("day07/input.txt")
        .expect("Could not read file input");

    println!("\n--- Day 7: Camel Cards ---");
    day07::run_a(&input);
    day07::run_b(&input);
}
