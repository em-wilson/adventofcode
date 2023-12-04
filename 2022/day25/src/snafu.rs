pub fn to_number(input:String) -> i64 {
    return input.chars()
        .collect::<Vec<_>>()
        .iter()
        .rev()
        .enumerate()
        .map(|(power, character)| 5_i64.pow(power as u32) * get_character_value(character) as i64)
        .sum();
}

pub fn from_number(input:i64) -> String {
    let mut digits:Vec<i64> = Vec::new();
    let mut next_result = input / 5;
    digits.push(input % 5);
    while next_result > 0 {
        digits.push(next_result % 5);
        next_result = next_result / 5;
    }

    let mut carry_forward = false;

    let result = digits.iter().map(|digit|{
        let mut result = *digit;
        if carry_forward {
            result += 1;
        }
        (result, carry_forward) = match result {
            5 => (0, true),
            3|4 => (result, true),
            _ => (result, false)
        };
        return convert_snafu_digit(result);
    }).collect::<Vec<_>>().iter().rev().map(|s|s.to_string()).collect::<String>();

    return match carry_forward {
        true => "1".to_string() + &result,
        _ => result
    };
}

fn convert_snafu_digit(digit:i64) -> String {
    return match digit {
        3 => "=".to_string(),
        4 => "-".to_string(),
        _ => digit.to_string()
    }
}

fn get_character_value(input:&char) -> i32 {
    return match input {
        &'=' => -2,
        &'-' => -1,
        _ => input.to_digit(10).unwrap() as i32
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_to_number() {
        assert_eq!(1747, to_number("1=-0-2".to_string()));
        assert_eq!(3, to_number("1=".to_string()));
        assert_eq!(37, to_number("122".to_string()));
        assert_eq!(100, to_number("1-00".to_string()));
        assert_eq!(314159265, to_number("1121-1110-1=0".to_string()));
        assert_eq!(40694673912, to_number("12=2=21=2-1=1122".to_string()));
    }

    #[test]
    fn test_from_number() {
        assert_eq!("1", from_number(1));
        assert_eq!("2", from_number(2));
        assert_eq!("1=", from_number(3));
        assert_eq!("1-", from_number(4));
        assert_eq!("10", from_number(5));
        assert_eq!("1-0", from_number(20));
        assert_eq!("111", from_number(31));
        assert_eq!("122", from_number(37));
        assert_eq!("1-00", from_number(100));
        assert_eq!("1=11-2", from_number(2022));
        assert_eq!("1-0---0", from_number(12345));
        assert_eq!("1121-1110-1=0", from_number(314159265));
    }
}