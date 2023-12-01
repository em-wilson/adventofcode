use std::collections::HashMap;

pub type NumberDictionary = HashMap<String, String>;

trait InsertRaw {
    fn insert_raw(&mut self, key:&'static str, value:&'static str);
}

impl InsertRaw for NumberDictionary {
    fn insert_raw(&mut self, key:&'static str, value:&'static str) {
        self.insert(key.to_string(), value.to_string());
    }
}

pub fn generate_dictionary(convert_strings: bool) -> NumberDictionary {
    return match convert_strings {
        true => create_wordy_dictionary(),
        false => create_dictionary()
    };
}

fn create_dictionary() -> NumberDictionary {
    let mut dict = NumberDictionary::new();
    for num in 0..=9 {
        dict.insert(num.to_string(), num.to_string());
    }
    return dict;
}

fn create_wordy_dictionary() -> NumberDictionary {
    let mut tokens = create_dictionary();
    tokens.insert_raw("zero", "0");
    tokens.insert_raw("one", "1");
    tokens.insert_raw("two", "2");
    tokens.insert_raw("three", "3");
    tokens.insert_raw("four", "4");
    tokens.insert_raw("five", "5");
    tokens.insert_raw("six", "6");
    tokens.insert_raw("seven", "7");
    tokens.insert_raw("eight", "8");
    tokens.insert_raw("nine", "9");
    return tokens;
}