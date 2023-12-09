use crate::number_dictionary::{NumberDictionary};
use crate::peekable_string::{PeekableString};

pub fn tokenize(input: String, token_dictionary: &NumberDictionary) -> Vec<String> {
    let mut first_number: Option<String> = None;
    let mut last_number: Option<String> = None;
    let peekable_string = PeekableString::create_from_string(input.to_string());
    let input_length = peekable_string.len();
    for i in 0..input_length {
        for (key, value) in token_dictionary {
            if first_number.is_none() {
                first_number = peekable_string.peek_number(key.to_string(), value.to_string(), i);
            }

            if last_number.is_none() {
                last_number = peekable_string.peek_number(key.to_string(), value.to_string(), input.len()-i-1)
            }

            if first_number.is_some() && last_number.is_some() {
                return [
                    first_number.unwrap().to_string(),
                    last_number.unwrap().to_string()
                ].to_vec();
            }
        }
    }
    panic!("should not have ended up here!");
}

#[cfg(test)]
mod tests {
    use crate::number_dictionary::{generate_dictionary, NumberDictionary};
    use super::tokenize;

    fn perform_test(input: &'static str, dictionary: &NumberDictionary, expected: [&str; 2]) {
        let result = tokenize(input.to_string(), dictionary);
        assert_eq!(expected.to_vec(), result);
    }

    #[test]
    fn test_tokenize_words_enabled() -> Result<(), String> {
        let dictionary = generate_dictionary(true);

        perform_test("1two3four", &dictionary, ["1","4"]);
        perform_test("eightwothree", &dictionary, ["8", "3"]);
        perform_test("jzxv89two8bjkmqmngkgtwotnmdqeightwonrc", &dictionary, ["8", "2"]);
        
        Ok(())
    }

    #[test]
    fn test_tokenize_words_disabled() -> Result<(), String> {
        let dictionary = generate_dictionary(false);

        perform_test("1two3four", &dictionary, ["1","3"]);
        
        Ok(())
    }
}