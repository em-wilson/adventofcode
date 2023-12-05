use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for part A: {}", calibrate(input.to_string(), 4));

    println!("Results for part B: {}", calibrate(input.to_string(), 14));
}

fn calibrate(input:String, marker_size:usize) -> usize {
    for (start, window) in input.chars().collect::<Vec<_>>().windows(marker_size).enumerate() {
        let mut dedup = window.iter().collect::<Vec<_>>();
        dedup.sort();
        dedup.dedup();
        if dedup.len() == marker_size {
            return start + marker_size;
        }
    }
    panic!("No windows with start-of-packet marker found");
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_calibrate_pos(input:&str) -> usize {
        return calibrate(input.to_string(), 4);
    }

    fn test_calibrate_message_pos(input:&str) -> usize {
        return calibrate(input.to_string(), 14);
    }

    #[test]
    fn test_calibrate_start() {
        assert_eq!(5, test_calibrate_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(6, test_calibrate_pos("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(10, test_calibrate_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(11, test_calibrate_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }

    #[test]
    fn test_calibrate_message() {
        assert_eq!(19, test_calibrate_message_pos("mjqjpqmgbljsphdztnvjfqwrcgsmlb"));
        assert_eq!(23, test_calibrate_message_pos("bvwbjplbgvbhsrlpgdmjqwftvncz"));
        assert_eq!(23, test_calibrate_message_pos("nppdvjthqldpwncqszvftbrmjlhg"));
        assert_eq!(29, test_calibrate_message_pos("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"));
        assert_eq!(26, test_calibrate_message_pos("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"));
    }
}