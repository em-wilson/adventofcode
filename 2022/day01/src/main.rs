use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    let elves = input.split("\n\n");

    let calories = elves
        .map(|elf| elf.split("\n")
            .map(|calories| calories.parse::<u32>().unwrap())
            .sum::<u32>()
        );

    let max_calories = calories.clone().max().unwrap();
    let mut top_elves = calories.clone().collect::<Vec<_>>();
    top_elves.sort();
    let top_elves_calories: u32 = top_elves.into_iter().rev().take(3).sum();

    println!("The elf with the most calories has {}", max_calories);

    println!("The elves with the top 3 most calories have a combined {}", top_elves_calories);
}
