mod bookend_tokenizer;
mod number_dictionary;
mod peekable_string;
mod trebuchet;

pub fn run_a(input:&str) {
    println!("Results for part A: {}", trebuchet::calibrate_input(input.to_string(), false));
}

pub fn run_b(input:&str) {
    println!("Results for part B: {}", trebuchet::calibrate_input(input.to_string(), true));
}
