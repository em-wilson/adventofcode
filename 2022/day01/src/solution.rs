pub fn most_calories(input:&str) -> usize {
    parse_calories(input).into_iter()
        .max()
        .unwrap()
}

pub fn combined_most_calories(input:&str) -> usize {
    let mut calories = parse_calories(input);
    calories.sort();
    calories.into_iter().rev().take(3).sum()
}

pub fn parse_calories(input:&str) -> Vec<usize> {
    input.split("\n\n")
        .map(|elf| elf.split("\n")
            .map(|calories| calories.parse::<usize>().unwrap())
            .sum()
        )
        .collect()
}