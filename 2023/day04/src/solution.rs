use std::collections::{HashMap, HashSet};

pub fn parse_card_input(input: String) -> Vec<Card> {
    return input.split("\n")
        .map(|input_line| Card::parse(input_line.to_string()))
        .collect::<Vec<_>>();
}

pub fn sum_card_values(input:&Vec<Card>) -> u32 {
    let mut value_generator = CardValueGenerator::new();
    return input
        .iter()
        .map(|card| value_generator.calculate(card))
        .sum();
}

pub fn explode_and_count_cards(input:&Vec<Card>) -> u32 {
    let mut cards: HashMap<u32, u32> = HashMap::new();
    let mut cards_processed = 0;

    for card in input {
        let resulting_cards = card.get_resulting_card_numbers();
        let mut multiplier = 1;
        if let Some(won_cards) = cards.get(&card.number).cloned() {
            multiplier += won_cards;
        }

        cards_processed += multiplier;

        for i in resulting_cards {
            if i <= card.number {
                panic!("Can't happen");
            }
            cards.entry(i).and_modify(|counter| *counter += multiplier).or_insert(multiplier);
        }
    }

    return cards_processed;
}

fn extract_number_vec(input:String) -> Vec<u32> {
    return input.split(" ")
        .filter(|str|str.parse::<u32>().is_ok() )
        .map(|str|str.parse::<u32>().unwrap())
        .collect();
}

pub struct CardValueGenerator {
    results: HashMap<u32, u32>,
}

impl CardValueGenerator {
    pub fn new() -> CardValueGenerator {
        CardValueGenerator{
            results: HashMap::new()
        }
    }

    pub fn calculate(&mut self, card:&Card) -> u32 {
        let win_count = card.winning_numbers.len() as u32;
        if win_count > 0 {
            if let Some(result) = self.results.get(&(win_count - 1)) {
                return *result;                
            }
            let result = 2_u32.pow(win_count - 1);
            self.results.insert(win_count -1, result);
            return result;
        }
        return 0;
    }
}

#[derive(Clone)]
pub struct Card {
    number: u32,
    winning_numbers: Vec<u32>,
}

impl Card {
    pub fn parse(input: String) -> Card {
        let envelope:Vec<_> = input.split(":").collect();
        let number_parts:Vec<_> = envelope[1].split("|").collect();
        let drawn_numbers = extract_number_vec(number_parts[0].to_string());
        let card_numbers = extract_number_vec(number_parts[1].to_string());
        return Card {
            number: *extract_number_vec(envelope[0].to_string()).first().unwrap(),
            winning_numbers: get_vector_intersection(drawn_numbers.clone(), card_numbers.clone()),
        }
    }

    pub fn get_resulting_card_numbers(&self) -> Vec<u32> {
        let mut results = Vec::new();
        if self.winning_numbers.len() > 0 {
            for i in self.number+1..self.number+self.winning_numbers.len() as u32+1 as u32 {
                results.push(i);
            }
        }
        return results;
    }
}

fn get_vector_intersection(input_left:Vec<u32>, input_right:Vec<u32>) -> Vec<u32> {
    let left:HashSet<_> = input_left.iter().collect::<HashSet<_>>();
    let right:HashSet<_> = input_right.iter().collect::<HashSet<_>>();
    return left.intersection(&right)
        .collect::<Vec<_>>()
        .iter()
        .map(|a| ***a)
        .collect::<Vec<_>>();
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
        assert_eq!(4, card.winning_numbers.len());
    }

    #[test]
    fn test_card_value() {
        assert_eq!(8, CardValueGenerator::new().calculate(&Card::parse("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string())));
        assert_eq!(2, CardValueGenerator::new().calculate(&Card::parse("Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19".to_string())));
        assert_eq!(2, CardValueGenerator::new().calculate(&Card::parse("Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1".to_string())));
        assert_eq!(1, CardValueGenerator::new().calculate(&Card::parse("Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string())));
        assert_eq!(0, CardValueGenerator::new().calculate(&Card::parse("Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36".to_string())));
        assert_eq!(0, CardValueGenerator::new().calculate(&Card::parse("Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string())));
    }
}