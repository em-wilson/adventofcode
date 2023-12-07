pub fn parse_seeds(input: &str) -> Vec<u64> {
    let seed_parts = input.split(": ").collect::<Vec<_>>();
    return seed_parts[1]
        .split(" ")
        .collect::<Vec<_>>()
        .iter()
        .map(|seed| seed.parse::<u64>().unwrap())
        .collect();
}

pub fn is_seed_line(input: &str) -> bool {
    let parts:Vec<_> = input.split(":").collect();
    return parts[0] == "seeds";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_seed_line() {
        assert_eq!(true, is_seed_line("seeds: 79 14 55 13"));
        assert_eq!(false, is_seed_line("seed-to-soil map:"));
    }

    #[test]
    fn test_parse_seeds() {
        assert_eq!(vec![79, 14, 55, 13], parse_seeds("seeds: 79 14 55 13"));
    }
}