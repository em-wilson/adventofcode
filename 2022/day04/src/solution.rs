use regex::Regex;
use std::collections::HashSet;

#[derive(Clone,Copy)]
pub enum OverlapType {
    Partial,
    Completely
}

impl OverlapType {
    fn check_ranges_overlap(self, a1:u32, a2:u32, b1:u32, b2:u32) -> bool {
        return match self {
            OverlapType::Completely => check_ranges_overlap_completely(a1,a2,b1,b2),
            OverlapType::Partial => check_ranges_overlap_partially(a1,a2,b1,b2),
        }
    }
}

pub fn count_overlaps(input:&str, overlap_type:OverlapType) -> usize {
    return input.split("\n")
        .filter(|line| check_line_contains_overlap(line.to_string(), overlap_type))
        .collect::<Vec<_>>()
        .len();
}

fn check_line_contains_overlap(input:String, overlap_type:OverlapType) -> bool {
    let re = Regex::new(r"(\d+)").unwrap();
    let nums :Vec<_> = re.find_iter(input.as_str())
        .map(|cap| cap.as_str().parse::<u32>().unwrap())
        .collect();
    return overlap_type.check_ranges_overlap(nums[0], nums[1], nums[2], nums[3]);
}

fn check_ranges_overlap_completely(a1:u32, a2:u32, b1:u32, b2:u32) -> bool {
    if a1 <= b1 && a2 >= b2 {
        return true;
    }

    if b1 <= a1 && b2 >= a2 {
        return true;
    }

    return false;
}

fn check_ranges_overlap_partially(a1:u32, a2:u32, b1:u32, b2:u32) -> bool {
    let left:HashSet<_> = (a1..=a2).collect();
    let right:HashSet<_> = (b1..=b2).collect();

    let results = left.intersection(&right).collect::<Vec<_>>();

    return results.len() > 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_overlaps_contain() {
        assert_eq!(2, count_overlaps("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8", OverlapType::Completely));
    }

    #[test]
    fn test_count_overlaps_slightly() {
        assert_eq!(4, count_overlaps("2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8", OverlapType::Partial));
    }
}