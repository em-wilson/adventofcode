use regex::{Regex};
use std::collections::{HashMap, HashSet};
use std::fs;


fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    let cards = parse_card_input(input.to_string());
    println!("Results for part A: {}", sum_card_values(&cards));
    println!("Results for part B: {}", explode_and_count_cards(&cards));
}

fn parse_card_input(input: String) -> Vec<Card> {
    return input.split("\n")
        .map(|input_line| Card::parse(input_line.to_string()))
        .collect::<Vec<_>>();
}

fn sum_card_values(input:&Vec<Card>) -> u32 {
    return input
        .iter()
        .map(|card| card.value())
        .sum();
}

fn explode_and_count_cards(input:&Vec<Card>) -> u32 {
    let mut cards = HashMap::new();
    let mut cards_to_process = Vec::new();
    let mut cards_processed = 0;
    for card in input {
        cards.insert(card.number, card.clone());
        cards_to_process.push(card.number);
    }

    while let Some(card_number) = cards_to_process.pop() {
        if let Some(card) = cards.get(&card_number) {
            cards_processed += 1;
            for i in card.get_resulting_card_numbers() {
                if i <= card_number {
                    panic!("Can't happen");
                }
                cards_to_process.push(i);
            }
        }
    }

    return cards_processed;
}

fn extract_number_vec(input:String) -> Vec<u32> {
    let re = Regex::new(r"(\d+)").unwrap();
    return re.find_iter(input.as_str())
        .map(|cap| cap.as_str().parse::<u32>().unwrap())
        .collect();
}

#[derive(Clone)]
struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
    card_numbers: Vec<u32>,
}

impl Card {
    pub fn parse(input: String) -> Card {
        let envelope:Vec<_> = input.split(":").collect();
        let number_parts:Vec<_> = envelope[1].split("|").collect();
        let winning_numbers = number_parts[0].to_string();
        let card_numbers = number_parts[1].to_string();
        return Card {
            number: *extract_number_vec(envelope[0].to_string()).first().unwrap(),
            winning_numbers: extract_number_vec(winning_numbers.to_string()),
            card_numbers: extract_number_vec(card_numbers.to_string()),
        }
    }

    pub fn value(&self) -> u32 {
        let left:HashSet<_> = self.winning_numbers.iter().collect::<HashSet<_>>();
        let right:HashSet<_> = self.card_numbers.iter().collect::<HashSet<_>>();
        let winners = left.intersection(&right).collect::<Vec<_>>();
        if winners.len() > 0 {
            return 2_u32.pow(winners.len() as u32 - 1);
        }
        return 0;
    }

    pub fn get_resulting_card_numbers(&self) -> Vec<u32> {
        let mut results = Vec::new();
        let left:HashSet<_> = self.winning_numbers.iter().collect::<HashSet<_>>();
        let right:HashSet<_> = self.card_numbers.iter().collect::<HashSet<_>>();
        let winners = left.intersection(&right).collect::<Vec<_>>();
        if winners.len() > 0 {
            for i in self.number+1..self.number+winners.len() as u32+1 as u32 {
                results.push(i);
            }
        }
        return results;
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_card_sums() {
        let cards = parse_card_input("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\nCard 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\nCard 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\nCard 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\nCard 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\nCard 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string());
        assert_eq!(13, sum_card_values(&cards));
        assert_eq!(30, explode_and_count_cards(&cards));
    }

    #[test]
    fn test_resulting_card_numbers() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        let results = card.get_resulting_card_numbers();
        assert_eq!(4, results.len());
    }

    #[test]
    fn test_card_parse() {
        let card = Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string());
        assert_eq!(1, card.number);
        assert_eq!(5, card.winning_numbers.len());
        assert_eq!(8, card.card_numbers.len());
    }

    #[test]
    fn test_card_value() {
        assert_eq!(8, Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string()).value());
        assert_eq!(2, Card::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string()).value());
        assert_eq!(2, Card::parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string()).value());
        assert_eq!(1, Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string()).value());
        assert_eq!(0, Card::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string()).value());
        assert_eq!(0, Card::parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string()).value());
    }
}