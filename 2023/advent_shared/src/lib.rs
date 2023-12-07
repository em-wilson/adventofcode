use std::fs;

pub struct AdventChallenge
{
    pub title: String,
    pub part_a: Box<dyn Fn(&str) -> String>,
    pub part_b: Box<dyn Fn(&str) -> String>,
}

impl AdventChallenge
{
    pub fn run(&self, input:&str) {
        let input = match input {
            "" => input.to_string(),
            _  => fs::read_to_string(input).expect("Could not read file input")
        };

        println!("\n{}", self.title);
        timer("Results for part A", &input, &self.part_a);
        timer("Results for part B", &input, &self.part_b);
    }
}

fn timer(title: &str, input: &str, f: &Box<dyn Fn(&str) -> String>) {
    let start = std::time::Instant::now();
    let result = f(input);
    println!("  {}: {} ({:?})", title, result, start.elapsed());
}