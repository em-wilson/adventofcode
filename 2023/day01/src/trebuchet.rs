use crate::bookend_tokenizer::tokenize;
use crate::number_dictionary::{generate_dictionary, NumberDictionary};
use rayon::prelude::*;

pub fn calibrate_input(input:&str, convert_strings: bool) -> u32 {
    let dictionary = generate_dictionary(convert_strings);
    
    input.split("\n")
        .collect::<Vec<_>>()
        .par_iter()
        .map(|line| calibrate_line(line.to_string(), &dictionary))
        .sum()
}

fn calibrate_line(calibration_line:String, dictionary: &NumberDictionary) -> u32 {
    let numbers = tokenize(calibration_line.to_string(), dictionary);

    return [numbers.first().unwrap().to_string(), numbers.last().unwrap().to_string()]
        .join("")
        .parse()
        .unwrap();
}

#[cfg(test)]
mod tests {
    use super::{calibrate_input};

    fn perform_test(input: &'static str, test_against_full_words: bool, expected_result: u32) {
        let result = calibrate_input(input.to_string(), test_against_full_words);
        assert_eq!(expected_result, result);
    }

    #[test]
    fn test_calibrate_input() -> Result<(), String> {
        perform_test("1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet", false, 142);

        Ok(())
    }

    #[test]
    fn test_calibrate_input_with_tokens() -> Result<(), String> {
        perform_test("two1nine", true, 29);
        perform_test("eightwothree", true, 83);
        perform_test("abcone2threexyz", true, 13);
        perform_test("xtwone3four", true, 24);
        perform_test("4nineeightseven2", true, 42);
        perform_test("zoneight234", true, 14);
        perform_test("7pqrstsixteen", true, 76);
        perform_test("9vvcsgxq", true, 99);

        Ok(())
    }

    #[test]
    fn test_calibrate_line() -> Result<(), String> {
        perform_test("1hey person3", false, 13);
        perform_test("1abc2", false, 12);
        perform_test("pqr3stu8vwx", false, 38);
        perform_test("a1b2c3d4e5f", false, 15);
        perform_test("treb7uchet", false, 77);
        perform_test("znoneight82lbghbsdktoneoneeight", false, 82);
        
        Ok(())
    }
}
