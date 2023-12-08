use std::collections::HashMap;

struct Map {
    left_indexes: HashMap<String, String>,
    right_indexes: HashMap<String, String>
}

impl Map {
    fn parse(input:&str) -> Map {
        let mut map = Map {
            left_indexes: HashMap::new(),
            right_indexes: HashMap::new()
        };
        for direction in input.split("\n") {
            let parts: Vec<_> = direction.split(" = ").collect();
            let index = parts[0];
            let (left_result, right_result) = parse_directions(parts[1]);
            map.left_indexes.insert(index.to_string(), left_result.to_string());
            map.right_indexes.insert(index.to_string(), right_result.to_string());
        }

        return map;
    }

    fn traverse(&self, steps:&Vec<char>, starting_step:&String, suffix_required:&String) -> i64 {
        let mut step_count = 0;
        let mut instruction_index = 0;
        let mut current_step_name = starting_step.to_string();


        loop {
            if instruction_index >= steps.len() {
                instruction_index = 0;
            }
            let instruction = steps[instruction_index];

            let next_step_opt = match instruction {
                'L' => self.left_indexes.get(current_step_name.as_str()),
                'R' => self.right_indexes.get(current_step_name.as_str()),
                _   => panic!("Invalid direction {}", instruction)
            };

            if let Some(next_step_name) = next_step_opt {
                current_step_name = next_step_name.to_string();
            } else {
                panic!("Invalid step name");
            }

            instruction_index += 1;
            step_count += 1;

            if current_step_name.ends_with(suffix_required) {
                return step_count as i64;
            }
        }
    }
}

fn parse_directions(input:&str) -> (String, String) {
    let mut chars = input.chars();
    chars.next();
    chars.next_back();
    let directions:Vec<_> = chars.as_str().split(", ").collect();
    return (directions[0].to_string(), directions[1].to_string());
}

pub fn count_steps(map_input:&str) -> i64 {
    let map_parts: Vec<_> = map_input.split("\n\n").collect();
    let steps: Vec<_> = map_parts[0].chars().collect();
    let map = Map::parse(map_parts[1]);
    let step_count = map.traverse(&steps, &String::from("AAA"), &String::from("ZZZ"));
    return step_count;
}

pub fn count_simultaneous_steps(map_input:&str) -> i64 {
    let map_parts: Vec<_> = map_input.split("\n\n").collect();
    let steps: Vec<_> = map_parts[0].chars().collect();
    let map = Map::parse(map_parts[1]);

    let mut starting_locations:Vec<_> = map.left_indexes.keys().into_iter().filter(|s|s.ends_with("A")).collect();
    let mut additional_starting_locations:Vec<_> = map.right_indexes.keys().filter(|s|s.ends_with("A")).into_iter().collect();
    starting_locations.append(&mut additional_starting_locations);

    let mut dedup = starting_locations.clone();
    dedup.sort();
    dedup.dedup();

    let step_counts_vec = dedup.into_iter().map(|step|map.traverse(&steps, step, &String::from("Z"))).collect::<Vec<_>>();
    let step_counts:&[i64] = step_counts_vec.as_slice();

    let mut result:i64 = 1;
    for step in step_counts {
        result = num::integer::lcm(result, *step);
    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn parse_test_input(test_file:&str) -> String {
        let input = fs::read_to_string(test_file)
            .expect("Could not read file input");

        return input.to_string();
    }
    
    #[test]
    fn test_direct_path() {
        let input = parse_test_input("test_input_a.txt");
        assert_eq!(2, count_steps(&input));
    }

    #[test]
    fn test_indirect_path() {
        let input = parse_test_input("test_input_b.txt");
        assert_eq!(6, count_steps(&input));
    }

    #[test]
    fn test_simultaneous_path() {
        let input = parse_test_input("test_input_c.txt");
        assert_eq!(6, count_simultaneous_steps(&input));
    }
}