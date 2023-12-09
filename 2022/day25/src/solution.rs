use crate::snafu;

pub fn sum_fuel_inputs(input:&str) -> i64 {
    return input.split("\n")
        .map(|s| snafu::to_number(s.to_string()))
        .collect::<Vec<i64>>()
        .iter()
        .sum();
}
