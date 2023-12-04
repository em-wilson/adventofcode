use crate::letter::letter_value;
use std::collections::HashSet;

fn get_duplicate_letter(input:String) -> char {
    let (left, right) = input.split_at(input.len() / 2);
    let unique_left = left.chars().collect::<HashSet<_>>();
    let unique_right = right.chars().collect::<HashSet<_>>();
    let intersection = unique_left.intersection(&unique_right).collect::<Vec<_>>();
    assert!(intersection.len() == 1, "Intersection must contain one character.");
    return **intersection.first().unwrap();
}

pub fn sum_priority(input:String) -> u32 {
    let mut sum = 0;
    for line in input.split("\n") {
        let letter = get_duplicate_letter(line.to_string());
        sum += letter_value(letter);
    }
    return sum;
}



#[cfg(test)]
mod test {
    use super::{get_duplicate_letter, letter_value, sum_priority};

    #[test]
    fn test_sum_priority() {
        assert_eq!(54, sum_priority("vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()))
    }

    #[test]
    fn test_letter_value() {
        assert_eq!(1, letter_value('a'));
        assert_eq!(2, letter_value('b'));
        assert_eq!(26, letter_value('z'));
        assert_eq!(27, letter_value('A'));
        assert_eq!(52, letter_value('Z'));
    }

    #[test]
    fn test_duplicate_letter() {
        assert_eq!('p', get_duplicate_letter("vJrwpWtwJgWrhcsFMMfFFhFp".to_string()));
        assert_eq!('L', get_duplicate_letter("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL".to_string()));
        assert_eq!('P', get_duplicate_letter("PmmdzqPrVvPwwTWBwg".to_string()));
        assert_eq!('v', get_duplicate_letter("wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn".to_string()));
        assert_eq!('t', get_duplicate_letter("ttgJtRGJQctTZtZT".to_string()));
        assert_eq!('s', get_duplicate_letter("CrZsJsPPZsGzwwsLwLmpwMDw".to_string()));
    }
}