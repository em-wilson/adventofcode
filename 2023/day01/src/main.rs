use std::fs;

pub mod bookend_tokenizer;
pub mod number_dictionary;
pub mod peekable_string;
pub mod trebuchet;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Calibrated input.txt: {}", trebuchet::calibrate_input(input, true));
}