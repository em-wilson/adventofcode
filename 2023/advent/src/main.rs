use colored::Colorize;

fn main() {
    let start = std::time::Instant::now();

    day01::create_challenge().run("day01/input.txt");
    day02::create_challenge().run("day02/input.txt");
    day03::create_challenge().run("day03/input.txt");
    day04::create_challenge().run("day04/input.txt");
    day05::create_challenge().run("day05/input.txt");
    day06::create_challenge().run("");
    day07::create_challenge().run("day07/input.txt");
    day08::create_challenge().run("day08/input.txt");

    println!("{} {}",
        "Total time:".bright_white(),
        format!("{:?}", start.elapsed()).cyan()
    );
}
