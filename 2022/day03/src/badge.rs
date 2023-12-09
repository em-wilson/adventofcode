use crate::letter::letter_value;
use std::collections::HashSet;

pub fn sum_priority(input: &str) -> u32 {
    return input.split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|group| badge_from_group(group.to_vec()))
        .map(|badge| letter_value(badge))
        .sum();
}

pub fn badge_from_group(input: Vec<&str>) -> char {
    let mut left:HashSet<_> = input[0].chars().collect::<HashSet<_>>();
    for result in input {
        let right = result.chars().collect::<HashSet<_>>();
        left = left.intersection(&right).copied().collect::<HashSet<_>>();
    }
    let results = left.into_iter().collect::<Vec<_>>();
    if results.len() != 1 {
        panic!("Did not expect a group with no badge");
    }
    return results[0];
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_priority() {
        assert_eq!(18, sum_priority("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg"));
        assert_eq!(70, sum_priority("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw"));
    }
}