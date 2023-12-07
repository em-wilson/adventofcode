use std::cmp::Ordering;
use std::collections::HashMap;

pub fn determine_winnings(input:&str, jokers_wild: bool) -> u32 {
    let mut hands: Vec<CamelHand> = input.split("\n")
        .map(|line|{
            let parts = line.split(" ").collect::<Vec<_>>();
            let cards = parts[0];
            let bet = parts[1].parse::<u32>().unwrap();
            CamelHand::new(cards, bet, jokers_wild)
        }).collect();
    hands.sort();

    return hands.into_iter().enumerate()
        .map(|(i, hand)|{
            let multiplier = i as u32 + 1;
            // println!("{} {}", multiplier, hand.cards);
            hand.bet * multiplier
        })
        .sum();
    
    // return hands.len() as u32;
}

#[derive(Debug, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind     = 7,
    FourOfAKind     = 6,
    FullHouse       = 5,
    ThreeOfAKind    = 4,
    TwoPair         = 3,
    OnePair         = 2,
    HighCard        = 1
}

impl HandType {
    pub fn from_cards(cards:&str, jokers_wild:bool) -> HandType {
        let drawn_type = HandType::from_cards_straight(cards);

        if jokers_wild && cards.contains("J") {
            if let Some(joker_type) = HandType::from_cards_circus(cards) {
                if joker_type > drawn_type {
                    return joker_type;
                }
            }
        }

        return drawn_type;
    }

    fn from_cards_straight(cards:&str) -> HandType {
        let chars: Vec<_> = cards.chars().collect();

        let mut card_counts: HashMap<char, u32> = HashMap::new();
        for card in chars.into_iter() {
            card_counts.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
        }

        let mut values = card_counts.values().collect::<Vec<_>>();
        values.sort_by(|a, b| b.cmp(a));
        let values = values.into_iter().cloned().collect::<Vec<u32>>();
        if values[0] == 5 {
            return HandType::FiveOfAKind;
        } else if values[0] == 4 {
            return HandType::FourOfAKind;
        } else if values[0] == 3 && values[1] == 2 {
            return HandType::FullHouse;
        } else if values[0] == 3 {
            return HandType::ThreeOfAKind;
        } else if values[0] == 2 && values[1] == 2 {
            return HandType::TwoPair;
        } else if values[0] == 2 {
            return HandType::OnePair;
        } else {
            return HandType::HighCard;
        }
    }

    fn from_cards_circus(cards:&str) -> Option<HandType> {
        let chars: Vec<_> = cards.chars().collect();

        let mut card_counts: HashMap<char, u32> = HashMap::new();
        for card in chars.into_iter() {
            card_counts.entry(card).and_modify(|counter| *counter += 1).or_insert(1);
        }

        if let Some(jokers) = card_counts.get(&'J').cloned() {
            card_counts.remove(&'J');
            let mut values = card_counts.values().collect::<Vec<_>>();
            values.sort_by(|a, b| b.cmp(a));
            let values = values.into_iter().cloned().collect::<Vec<u32>>();

            if jokers == 5 {
                return Some(HandType::FiveOfAKind);
            } else if values[0]+jokers == 5 {
                return Some(HandType::FiveOfAKind);
            } else if values[0]+jokers == 4 {
                return Some(HandType::FourOfAKind);
            } else if values[0]+jokers == 3 && values[1] == 2 {
                return Some(HandType::FullHouse);
            } else if values[0]+jokers == 3 {
                return Some(HandType::ThreeOfAKind);
            } else if values[0]+jokers == 2 && values[1] == 2 {
                return Some(HandType::TwoPair);
            } else if values[0]+jokers == 2 {
                return Some(HandType::OnePair);
            } else {
                return Some(HandType::HighCard);
            }
        }
        return None;
    }
    
}

struct CamelHand {
    hand_type: HandType,
    cards: String,
    bet: u32,
    jokers_wild: bool,
}

impl CamelHand {
    pub fn new(cards:&str, bet:u32, jokers_wild: bool) -> CamelHand {
        let hand_type = HandType::from_cards(cards, jokers_wild);
        CamelHand{
            hand_type: hand_type,
            cards: cards.to_string(),
            bet: bet,
            jokers_wild: jokers_wild,
        }
    }
}

impl Ord for CamelHand {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.hand_type == other.hand_type {
            let other_cards = map_card_values(other.cards.to_string(), other.jokers_wild);
            for (i, card) in map_card_values(self.cards.to_string(), self.jokers_wild).into_iter().enumerate() {
                if card > other_cards[i] {
                    return Ordering::Greater;
                }
                if card < other_cards[i] {
                    return Ordering::Less;
                }
            }
        }
        
        if self.hand_type < other.hand_type {
            return Ordering::Less;
        }
        return Ordering::Greater;
    }
}

impl PartialOrd for CamelHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for CamelHand {}

impl PartialEq for CamelHand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

fn map_card_values(cards:String, jokers_wild: bool) -> Vec<u32> {
    let joker_value = match jokers_wild {
        true => 1,
        _ => 11
    };

    return cards.chars().into_iter()
        .map(|card|match card {
            'T' => 10,
            'J' => joker_value,
            'Q' => 12,
            'K' => 13,
            'A' => 14,
            _ => card.to_digit(10).unwrap()
        })
        .collect::<Vec<_>>();
}

#[cfg(test)]
mod part_a_test {
    use super::*;

    #[test]
    fn test_determine_winnings() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(6440, determine_winnings(input, false));
    }

    #[test]
    fn test_compare_hands() {
        assert_eq!(false, CamelHand::new("KTKKK", 684, false) < CamelHand::new("3J333", 483, false));
        assert_eq!(false, CamelHand::new("T55J5", 684, false) < CamelHand::new("32T3K", 765, false));
        assert_eq!(true, CamelHand::new("32T3K", 765, false) < CamelHand::new("T55J5", 684, false));
        assert_eq!(true, CamelHand::new("T55J5", 684, false) < CamelHand::new("QQQJA", 483, false));
        assert_eq!(true, CamelHand::new("T55J5", 684, false)< CamelHand::new("QQQJA", 483, false));
        assert_eq!(true, CamelHand::new("88J88", 684, false) < CamelHand::new("KTKKK", 483, false));
    }

    #[test]
    fn test_hand_from_cards() {
        assert_eq!(HandType::FiveOfAKind, HandType::from_cards("33333", false));
        assert_eq!(HandType::FourOfAKind, HandType::from_cards("33332", false));
        assert_eq!(HandType::FiveOfAKind, HandType::from_cards("AAAAA", false));
        assert_eq!(HandType::FourOfAKind, HandType::from_cards("AA8AA", false));
        assert_eq!(HandType::FullHouse, HandType::from_cards("23332", false));
        assert_eq!(HandType::ThreeOfAKind, HandType::from_cards("TTT98", false));
        assert_eq!(HandType::TwoPair, HandType::from_cards("23432", false));
        assert_eq!(HandType::OnePair, HandType::from_cards("A23A4", false));
        assert_eq!(HandType::HighCard, HandType::from_cards("23456", false));
    }

    #[test]
    fn test_hand_compare() {
        assert_eq!(true, HandType::ThreeOfAKind >= HandType::TwoPair);
        assert_eq!(true, HandType::FourOfAKind == HandType::FourOfAKind);
        assert_eq!(true, HandType::FiveOfAKind > HandType::FullHouse);
        assert_eq!(true, HandType::HighCard < HandType::OnePair);
    }
}

#[cfg(test)]
mod part_b_test {
    use super::*;

    #[test]
    fn test_determine_winnings() {
        let input = "32T3K 765\nT55J5 684\nKK677 28\nKTJJT 220\nQQQJA 483";
        assert_eq!(5905, determine_winnings(input, true));
    }

    #[test]
    fn test_joker_hands() {
        assert_eq!(HandType::OnePair, HandType::from_cards("32T3K", true));
        assert_eq!(HandType::FourOfAKind, HandType::from_cards("T55J5", true));
        assert_eq!(HandType::TwoPair, HandType::from_cards("KK677", true));
        assert_eq!(HandType::FourOfAKind, HandType::from_cards("KTJJT", true));
        assert_eq!(HandType::FourOfAKind, HandType::from_cards("QQQJA", true));
    }
}