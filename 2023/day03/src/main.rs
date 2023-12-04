extern crate regex;
use regex::{Regex, Match};
use std::fs;

use crate::position::Pos;

pub mod position;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Sum for part A: {}", calculate_value_of_number_with_adjacent_symbols(input.to_string()));
    println!("Sum for part B: {}", calculate_value_of_gears(input.to_string()));
}

#[derive(Clone,Copy)]
struct PositionalSymbol {
    symbol: char,
    position: Pos,
}

struct PositionalNumber {
    num: i32,
    positions: Vec<Pos>,
}

impl PositionalNumber {
    fn from_capture(input:Match, y:i32) -> PositionalNumber {
        return PositionalNumber {
            num: input.as_str().parse::<i32>().unwrap(),
            positions: positions_from_range(input.start() as i32, input.len() as i32, y),
        }
    }

    fn has_adjacent_symbol(&self, symbols:&Vec<PositionalSymbol>) -> bool {
        for position in &self.positions {
            for symbol in symbols {
                let symbol_position = symbol.position;
                if position.is_adjacent_to(symbol_position) {
                    return true;
                }
            }
        }
        return false;
    }
}

fn positions_from_range(start:i32, length:i32, y: i32) -> Vec<Pos> {
    return (start..start+length)
        .map(|x| Pos::from(x as i32,y))
        .collect::<Vec<Pos>>()
}

fn extract_number_pos(input:String, y: i32) -> Vec<PositionalNumber> {
    let re = Regex::new(r"(\d+)").unwrap();
    return re.find_iter(input.as_str())
        .map(|cap| PositionalNumber::from_capture(cap, y))
        .collect();
}

fn extract_symbol_pos(input:String, y: i32) -> Vec<PositionalSymbol> {
    return input.chars()
        .enumerate()
        .filter(|(_, symbol)| symbol != &'.')
        .filter(|(_, symbol)| !symbol.is_digit(10))
        .map(|(x, symbol)| PositionalSymbol{symbol: symbol, position: Pos{x:x as i32,y:y}})
        .collect();
}

fn calculate_value_of_number_with_adjacent_symbols(input: String) -> i32 {
    let mut numbers : Vec<PositionalNumber> = Vec::new();
    let mut symbols : Vec<PositionalSymbol> = Vec::new();

    for (y, line) in input.split("\n").enumerate() {
        let mut new_numbers = extract_number_pos(line.to_string(), y as i32);
        let mut new_symbols = extract_symbol_pos(line.to_string(), y as i32);
        numbers.append(&mut new_numbers);
        symbols.append(&mut new_symbols);
    }

    return numbers
        .iter()
        .filter(|number| number.has_adjacent_symbol(&symbols))
        .map(|number| number.num )
        .sum();
}

fn calculate_value_of_gears(input: String) -> i32 {
    let mut numbers : Vec<PositionalNumber> = Vec::new();
    let mut symbols : Vec<PositionalSymbol> = Vec::new();

    for (y, line) in input.split("\n").enumerate() {
        let mut new_numbers = extract_number_pos(line.to_string(), y as i32);
        let mut new_symbols = extract_symbol_pos(line.to_string(), y as i32);
        numbers.append(&mut new_numbers);
        symbols.append(&mut new_symbols);
    }

    let potential_gears = symbols.iter().filter(|symbol| symbol.symbol == '*');

    let mut gear_sum = 0;

    for potential_gear in potential_gears {
        let mut gear_as_list : Vec<PositionalSymbol> = Vec::new();
        gear_as_list.push(*potential_gear);
        let adjacent_numbers:Vec<&PositionalNumber> = numbers.iter().filter(|number| number.has_adjacent_symbol(&gear_as_list)).collect();
        if adjacent_numbers.len() == 2 {
            let number1 = adjacent_numbers[0];
            let number2 = adjacent_numbers[1];
            gear_sum += number1.num * number2.num;
        }
    }

    return gear_sum;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_double_number_extraction() {
        let results = extract_number_pos("467..114..".to_string(), 1);
        assert_eq!(2, results.len());
        assert_eq!(467, results[0].num);
        assert_eq!(3, results[0].positions.len());
        assert_eq!(Pos::from(0,1), results[0].positions[0]);
        assert_eq!(Pos::from(1,1), results[0].positions[1]);
        assert_eq!(Pos::from(2,1), results[0].positions[2]);
        assert_eq!(114, results[1].num);
        assert_eq!(3, results[1].positions.len());
        assert_eq!(Pos::from(5,1), results[1].positions[0]);
        assert_eq!(Pos::from(6,1), results[1].positions[1]);
        assert_eq!(Pos::from(7,1), results[1].positions[2]);
    }

    #[test]
    fn test_single_number_extraction() {
        let results = extract_number_pos("..592.....".to_string(), 6);
        assert_eq!(1, results.len());
        assert_eq!(592, results[0].num);
        assert_eq!(3, results[0].positions.len());
        assert_eq!(Pos::from(2,6), results[0].positions[0]);
        assert_eq!(Pos::from(3,6), results[0].positions[1]);
        assert_eq!(Pos::from(4,6), results[0].positions[2]);
    }

    #[test]
    fn test_symbol_extraction() {
        let results = extract_symbol_pos("...$.*....".to_string(), 8);
        assert_eq!(2, results.len());
        assert_eq!('$', results[0].symbol);
        assert_eq!(Pos::from(3,8), results[0].position);
        assert_eq!('*', results[1].symbol);
        assert_eq!(Pos::from(5,8), results[1].position);
    }

    #[test]
    fn test_symbol_extraction_with_numbers() {
        let results = extract_symbol_pos(".....+.58.".to_string(), 5);
        assert_eq!(1, results.len());
        assert_eq!('+', results[0].symbol);
        assert_eq!(Pos::from(5,5), results[0].position);
    }

    #[test]
    fn test_has_adjacent_symbol() {
        let positions = (6..8).map(|pos| Pos::from(pos,2)).collect();
        let number = PositionalNumber{num:633, positions:positions};
        let mut symbols:Vec<PositionalSymbol> = Vec::new();
        symbols.push(PositionalSymbol{symbol:'😃', position: Pos::from(6,3)});
        assert_eq!(true, number.has_adjacent_symbol(&symbols));
    }

    #[test]
    fn test_does_not_has_adjacent_symbol() {
        let positions = (6..8).map(|pos| Pos::from(pos,2)).collect();
        let number = PositionalNumber{num:633, positions:positions};
        let symbols = (6..6).map(|x| PositionalSymbol{symbol:'😃', position: Pos::from(x,7)}).collect();
        assert_eq!(false, number.has_adjacent_symbol(&symbols));
    }

    #[test]
    fn test_calculate_value_of_number_with_adjacent_symbols() {
        let input = "467..114..\n...*......\n..35..633.";
        assert_eq!(502, calculate_value_of_number_with_adjacent_symbols(input.to_string()));
    }

    #[test]
    fn test_calculate_value_of_gears() {
        let input = "467..114..\n...*......\n..35..633.";
        assert_eq!(16345, calculate_value_of_gears(input.to_string()));

        let input = "467..114..\n...*......\n..35..633.\n......#...\n617*......\n.....+.58.\n..592.....\n......755.\n...$.*....\n.664.598..";
        assert_eq!(467835, calculate_value_of_gears(input.to_string()));
    }
}