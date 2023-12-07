use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs;
use std::str::FromStr;

pub mod shared;

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Could not read file input");

    println!("Results for A: {}", fetch_lowest_location(&input));
    println!("Results for B: {}", fetch_lowest_location_by_seed_ranges(&input));
}

fn fetch_lowest_location(input:&str) -> u64 {
    let almanac = parse_almanac(input);
    let result = almanac.seeds.clone().into_iter()
        .map(|seed| almanac.get_location_for_seed_range(vec!(seed,1)))
        .collect::<Vec<_>>().iter()
        .min().unwrap().clone();

    return result;
}

fn fetch_lowest_location_by_seed_ranges(input:&str) -> u64 {
    let almanac = parse_almanac(input);

    return almanac.seeds.chunks(2)
        .collect::<Vec<_>>().into_iter()
        .map(|seed| almanac.get_location_for_seed_range(seed.to_vec()))
        .collect::<Vec<_>>().iter()
        .min().unwrap().clone();
}

type AlamanacMappings = HashMap<AlmanacResourceType, AlamanacMapping>;

#[derive(Clone)]
struct AlamanacMapping {
    source_type: AlmanacResourceType,
    destination_type: AlmanacResourceType,
    ranges: AlmanacRanges,
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum AlmanacResourceType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location
}

impl FromStr for AlmanacResourceType {
    type Err = String;

    fn from_str(input: &str) -> Result<AlmanacResourceType, Self::Err> {
        match input {
            "seed"          => Ok(AlmanacResourceType::Seed),
            "soil"          => Ok(AlmanacResourceType::Soil),
            "fertilizer"    => Ok(AlmanacResourceType::Fertilizer),
            "water"         => Ok(AlmanacResourceType::Water),
            "light"         => Ok(AlmanacResourceType::Light),
            "temperature"   => Ok(AlmanacResourceType::Temperature),
            "humidity"      => Ok(AlmanacResourceType::Humidity),
            "location"      => Ok(AlmanacResourceType::Location),
            _               => Err(format!("Could not parse {}", input)),
        }
    }
}

struct Almanac {
    seeds:Vec<u64>,
    mappings:AlamanacMappings,
}

impl Almanac {
    fn from(seeds:Vec<u64>, mappings:AlamanacMappings) -> Almanac {
        return Almanac{
            seeds:seeds, 
            mappings: mappings
        };
    }

    fn get_location_for_seed_range(&self, range:Vec<u64>) -> u64 {
        let mut key_ranges = vec!(vec!(range));
        let mut source_type = AlmanacResourceType::Seed;
        while let Some(mapping) = self.mappings.get(&source_type) {
            // println!("{:?}: Smashing {:?}", mapping.destination_type, key_ranges);
            // println!("into {:?}", mapping.ranges);
            source_type = mapping.destination_type;
            key_ranges = key_ranges.clone().iter()
                .flat_map(|ranges|ranges.into_iter().map(|range|smash_intervals(range.clone(), mapping.ranges.clone()))).collect();
            // println!("And got {:?}", key_ranges);
            }

        // println!("And got {:?}", key_ranges);
        
        let result = key_ranges.clone().into_iter()
            .flat_map(|ranges|{
                ranges.into_iter().map(|range|range[0])
            })
            .min().unwrap();

        // println!("Min is {}", result);

        return result;
    }
}

#[derive(Clone, Debug)]
struct AlmanacRanges {
    ranges: Vec<AlmanacRange>,
}

impl Eq for AlmanacRanges {}

impl PartialEq for AlmanacRanges {
    fn eq(&self, other: &Self) -> bool {
        self.ranges == other.ranges
    }
}

impl Ord for AlmanacRanges {
    fn cmp(&self, other: &Self) -> Ordering {
        self.ranges.cmp(&other.ranges)
    }
}

impl PartialOrd for AlmanacRanges {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl AlmanacRanges {
    fn new() -> AlmanacRanges {
        return AlmanacRanges{
            ranges: vec![]
        };
    }

    fn get_lowest_range_above_threshold(&self, pos:u64) -> Option<AlmanacRange> {
        if self.ranges.len() == 0 {
            return None;
        }

        let mut found_range: Option<AlmanacRange> = None;

        self.ranges.iter().for_each(|range|{
            if range.source_end >= pos {
                if let Some(prev_range) = &found_range {
                    if prev_range.source_start < range.source_start {
                        return;
                    }
                }
                found_range = Some(range.clone());
            }
        });
        return found_range;
    }

    fn insert(&mut self, source_start:u64, destination_start:u64, range_length:u64) {
        self.ranges.push(AlmanacRange::new(source_start, destination_start, range_length));
        self.ranges.sort();
    }
}

#[derive(Clone, Debug)]
struct AlmanacRange {
    source_start: u64,
    source_end: u64,
    offset: i64,
}

impl Ord for AlmanacRange {
    fn cmp(&self, other: &Self) -> Ordering {
        self.source_start.cmp(&other.source_start)
    }
}

impl PartialOrd for AlmanacRange {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for AlmanacRange {}

impl PartialEq for AlmanacRange {
    fn eq(&self, other: &Self) -> bool {
        self.source_start == other.source_start
    }
}

impl AlmanacRange {
    fn new(source_start:u64, destination_start:u64, range_length:u64) -> AlmanacRange {
        return AlmanacRange{ 
            source_start: source_start.clone(),
            source_end: source_start + range_length as u64 - 1,
            offset: destination_start as i64 - source_start as i64,
        };
    }
}

fn parse_almanac(input: &str) -> Almanac {
    let mut seeds = Vec::new();
    let mut mappings = AlamanacMappings::new();
    for group in input.split("\n\n") {
        if shared::is_seed_line(group) {
            seeds = shared::parse_seeds(group)
        } else {
            let resource_group = parse_resource_group(group);
            mappings.insert(resource_group.source_type, resource_group);
        }
    }
    return Almanac::from(seeds.clone(), mappings.clone());
}

fn parse_resource_group(input: &str) -> AlamanacMapping {
    let lines:Vec<_> = input.split("\n").collect();
    let mapping_cols:Vec<_> = lines[0].split(" ").collect();
    let mapping_defs:Vec<_> = mapping_cols[0].split("-").collect();
    let mut ranges = AlmanacRanges::new();

    let source_type = AlmanacResourceType::from_str(mapping_defs[0]).unwrap();
    for line in &lines[1..lines.len()] {
        let inputs:Vec<_> = line.split(" ").collect::<Vec<_>>().iter().map(|num| num.parse::<u64>().unwrap()).collect();
        ranges.insert(inputs[1], inputs[0], inputs[2]);
    }

    return AlamanacMapping{
        source_type: source_type,
        destination_type: AlmanacResourceType::from_str(mapping_defs[2]).unwrap(),
        ranges: ranges
    };
}

fn smash_intervals(keys:Vec<u64>, mappings:AlmanacRanges) -> Vec<Vec<u64>> {
    let mut result = vec!();
    // println!("Smashing {}:{}", keys[0], keys[1]);

    let mut i = keys[0];
    let keys_end = keys[0]+keys[1]-1;
    while i <= keys_end {
        // println!("starting with {}", i);
        if let Some(lowest_range) = mappings.get_lowest_range_above_threshold(i) {
            // println!("Found a range {}:{} (offset: {})", lowest_range.source_start, lowest_range.source_end, lowest_range.offset);
            // println!("a0 {}", i);
            if lowest_range.source_start > i {
                let len = std::cmp::min(keys[0]+keys[1]-i, lowest_range.source_start-i);
                result.push(vec!(i, len));
                i = lowest_range.source_start;
                if i > keys_end {
                    continue;
                }
            }

            // println!("a {}", i);
            if lowest_range.source_start > keys_end {
                // println!("{} > {}", lowest_range.source_start, keys[0]+keys[1]-1);
                // println!("{}:{}", i, keys[0]+keys[1]-1);
                result.push(vec!(i, keys[0]+keys[1]-1));
                i = keys[0]+keys[1]-1;

                if i > keys_end {
                    continue;
                }
            }

            // println!("b {}", i);
            let lowest_range_start_offset = i as i64 - lowest_range.source_start as i64;
            // println!("getting start: {} - {} = {}", i, lowest_range.source_start, lowest_range_start_offset);
            let mapped_len = lowest_range.source_end as i64 + 1 - lowest_range.source_start as i64 - lowest_range_start_offset as i64;
            // println!("starting at {}", lowest_range_start_offset);
            let source_len = keys[0]+keys[1]-i;
            // println!("source len {}", source_len);
            // println!("mapped len {}", mapped_len);
            let len = std::cmp::min(mapped_len, source_len as i64);
            let mapped_start = i as i64 + lowest_range.offset as i64;
            // println!("{} + {} = {}", i, lowest_range.offset, mapped_start);
            result.push(vec!(mapped_start as u64, len as u64));
            i = i + len as u64 - 1;
            // println!("c {}", i);
        } else {
            // println!("No lower range {}", i);
            result.push(vec!(i, keys[0]+keys[1]-i));
            i = keys[0]+keys[1];
        }
        // println!("{}", i);
        i += 1;
    }

    // println!("Smashed {}:{}", keys[0], keys[1]);
    // println!("Against {:?}", mappings);
    // println!("getting {:?}", result);
    return result;
}

#[cfg(test)]
mod tests {
    use std::fs;
    use super::*;

    trait TestAlamacRange {
        fn from_vec(vec:Vec<AlmanacRange>) -> AlmanacRanges;
    }

    impl TestAlamacRange for AlmanacRanges {
        fn from_vec(vec:Vec<AlmanacRange>) -> AlmanacRanges {
            AlmanacRanges {
                ranges: vec.clone()
            }
        }
    }

    fn get_input() -> String {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file input");
        return input;
    }

    #[test]
    fn test_parse_input_maps() {
        let input = &get_input();
        let results = parse_almanac(input);
        assert_eq!(4, results.seeds.len());
        assert_eq!(7, results.mappings.len());
    }

    #[test]
    fn test_parse_resource_group() {
        let input = "water-to-light map:\n88 18 7";
        let results = parse_resource_group(input);
        assert_eq!(AlmanacResourceType::Water, results.source_type);
        assert_eq!(AlmanacResourceType::Light, results.destination_type);
    }

    #[test]
    fn test_almanac_type_from_string() {
        assert_eq!(AlmanacResourceType::Fertilizer, AlmanacResourceType::from_str("fertilizer").unwrap());
    }

    #[test]
    fn test_fetch_lowest_location() {
        let input = &get_input();
        assert_eq!(35, fetch_lowest_location(input));
    }

    #[test]
    fn test_fetch_lowest_location_by_seed_ranges() {
        let input = &get_input();
        assert_eq!(46, fetch_lowest_location_by_seed_ranges(input));
    }

    #[test]
    fn test_smash_keys() {
        assert_eq!(vec!(vec!(1,3), vec!(6,3), vec!(7,3), vec!(11,3)), smash_intervals(vec!(1,12), AlmanacRanges::from_vec(vec!(AlmanacRange::new(4,6,3), AlmanacRange::new(10,11,12)))));
        assert_eq!(vec!(vec!(11,7)), smash_intervals(vec!(11,7), AlmanacRanges::new()));
        assert_eq!(vec!(vec!(34,3), vec!(14,4)), smash_intervals(vec!(11,7), AlmanacRanges::from_vec(vec!(AlmanacRange::new(9,32,5)))));
        assert_eq!(vec!(vec!(5,1), vec!(23,2)), smash_intervals(vec!(5,3), AlmanacRanges::from_vec(vec!(AlmanacRange::new(6,23,4)))));
        assert_eq!(vec!(vec!(13,1)), smash_intervals(vec!(13,1), AlmanacRanges::from_vec(vec!(AlmanacRange::new(50, 97, 2)))));
        assert_eq!(vec!(vec!(2562500003,1)), smash_intervals(vec!(2168730372,1), AlmanacRanges::from_vec(vec!(AlmanacRange::new(1890108028, 2283877659, 353770946)))));
    }
}