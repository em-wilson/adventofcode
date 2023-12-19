use std::collections::HashMap;

pub fn sum_parts_rating(input:&str) -> usize {
    let (workflows, parts) = parse_input(&input);

    parts.iter()
        .filter(|p|process_part(&p, &workflows))
        .map(|p|p.x + p.m + p.a + p.s)
        .sum()
}

pub fn sum_parts_combinations(input:&str) -> usize {
    let (workflows, _) = parse_input(&input);
    
    get_valid_ranges(
        [(1, 4000), (1, 4000), (1, 4000), (1, 4000)],
        String::from("in"),
        &workflows
    )
    .into_iter()
    .map(get_combinations)
    .sum()
}

fn get_combinations(range:Range) -> usize {
    range.iter()
        .fold(1,|acc, &(start, end)|acc * (end - start + 1))

}

type Range = [(usize, usize); 4];

macro_rules! attr_to_idx {
    ($a: expr) => {
        match $a.as_str() {
            "x" => 0,
            "m" => 1,
            "a" => 2,
            "s" => 3,
             _  => panic!("what is {}", $a)
        }
    }
}

fn get_valid_ranges(current_range:Range, next_workflow:String, workflows:&HashMap<String, WorkFlow>) -> Vec<Range> {
    match next_workflow.as_str() {
        "A" => return vec![current_range],
        "R" => return vec![],
         _  => ()
    }

    let mut current_range = current_range;
    let mut valid_ranges = vec![];

    if let Some(wf) = workflows.get(&next_workflow) {
        for rule in &wf.rules {
            let destination = rule.next_workflow.clone();

            match rule.condition {
                RuleCondition::LessThan => {
                    if current_range[attr_to_idx!(rule.attribute)].1 < rule.value {
                        valid_ranges.extend(get_valid_ranges(current_range, destination, workflows));
                    } else if current_range[attr_to_idx!(rule.attribute)].0 < rule.value {
                        let mut new_range = current_range;
                        new_range[attr_to_idx!(rule.attribute)].1 = rule.value - 1;
                        valid_ranges.extend(get_valid_ranges(new_range, destination, workflows));
                        current_range[attr_to_idx!(rule.attribute)].0 = rule.value;
                    }
                },
                RuleCondition::GreaterThan => {
                    if current_range[attr_to_idx!(rule.attribute)].0 > rule.value {
                        valid_ranges.extend(get_valid_ranges(current_range, destination, workflows));
                    } else if current_range[attr_to_idx!(rule.attribute)].1 > rule.value {
                        let mut new_range = current_range;
                        new_range[attr_to_idx!(rule.attribute)].0 = rule.value + 1;
                        valid_ranges.extend(get_valid_ranges(new_range, destination, workflows));
                        current_range[attr_to_idx!(rule.attribute)].1 = rule.value;
                    }
                },
                RuleCondition::Default => {
                    valid_ranges.extend(get_valid_ranges(current_range, destination, workflows));
                }
            }
        }
    }
    valid_ranges
}

struct Part {
    x:usize,
    m:usize,
    a:usize,
    s:usize,
}

impl Part {
    fn new() -> Part {
        Part{x:0,m:0,a:0,s:0}
    }
    fn from(input:&str) -> Part {
        let mut chars = input.chars();
        chars.next();
        chars.next_back();
        let mut result = Part::new();
        for part in chars.as_str().split(",") {
            let parts:Vec<_> = part.split("=").collect();
            match parts[0] {
                "x" => result.x = parts[1].parse().unwrap(),
                "m" => result.m = parts[1].parse().unwrap(),
                "a" => result.a = parts[1].parse().unwrap(),
                "s" => result.s = parts[1].parse().unwrap(),
                 _  => panic!("Unknown part definition {}", part),
            }
        }
        result
    }
}

#[derive(Clone)]
enum RuleCondition {
    GreaterThan,    // >
    LessThan,       // <
    Default,        // No Matches
}

#[derive(Clone)]
struct Rule {
    attribute:String,
    condition:RuleCondition,
    value:usize,
    next_workflow:String,
}

impl Rule {
    fn from(input:&str) -> Rule {
        let condition:Vec<_> = input.split(":").collect();
        if condition.len() == 2 {
            let next_workflow = condition[1].to_string();
            let matching:Vec<_> = condition[0].split(">").collect();
            if matching.len() == 2 {
                Rule {
                    attribute: matching[0].to_string(),
                    condition: RuleCondition::GreaterThan,
                    value: matching[1].parse().unwrap(),
                    next_workflow: next_workflow
                }
            } else {
                let matching:Vec<_> = condition[0].split("<").collect();
                if matching.len() == 2 {
                    Rule {
                        attribute: matching[0].to_string(),
                        condition: RuleCondition::LessThan,
                        value: matching[1].parse().unwrap(),
                        next_workflow: next_workflow
                    }
                } else {
                    panic!("Unknown matching conditions {}", condition[0]);
                }
            }
        } else {
            Rule::straight_redirect(condition[0])
        }
    }

    fn straight_redirect(redirect_result:&str) -> Rule {
        Rule {
            attribute: String::from(""),
            condition: RuleCondition::Default,
            value: 0,
            next_workflow: redirect_result.to_string()
        }
    }
}

struct WorkFlow {
    name: String,
    rules: Vec<Rule>
}

impl WorkFlow {
    fn from(input:&str) -> WorkFlow {
        let parts:Vec<&str> = input.split("{").collect();
        let mut rules_str = parts[1].chars();
        rules_str.next_back();
        WorkFlow {
            name: String::from(parts[0]),
            rules: rules_str.as_str().split(",").map(|r|Rule::from(r)).collect(),
        }
    }

    fn process_part(&self, part:&Part) -> String {
        for rule in self.rules.clone() {
            match rule.condition {
                RuleCondition::GreaterThan | RuleCondition::LessThan => {
                    let comparer = match rule.attribute.as_str() {
                        "x" => part.x,
                        "m" => part.m,
                        "a" => part.a,
                        "s" => part.s,
                        _  => panic!("Unknown attribute: {}", rule.attribute)
                    };

                    let result = match rule.condition {
                        RuleCondition::GreaterThan => comparer > rule.value,
                        RuleCondition::LessThan => comparer < rule.value,
                        _ => panic!("Shoudl not come here")
                    };
                    if result {
                        return rule.next_workflow.to_string();
                    }
                },
                RuleCondition::Default => {
                    return rule.next_workflow.to_string();
                }
            }
        }
        panic!("No matching rules");
    }
}

fn process_part(part:&Part, workflows:&HashMap<String, WorkFlow>) -> bool {
    let mut next_workflow = String::from("in");
    while let Some(workflow) = workflows.get(&next_workflow) {
        next_workflow = workflow.process_part(part);
    }
    next_workflow == "A"
}

fn parse_input(input:&str) -> (HashMap<String, WorkFlow>, Vec<Part>) {
    let segments:Vec<&str> = input.split("\n\n").collect();
    let workflows:HashMap<String, WorkFlow> = segments[0].split("\n")
        .map(|l|{let w = WorkFlow::from(l);(w.name.to_string(),w)})
        .collect();
    let parts = segments[1].split("\n")
        .map(|l|Part::from(l))
        .collect();
    (workflows, parts)
}

#[cfg(test)]
mod test {
    use super::*;
    use std::fs;

    #[test]
    fn test_sum_parts_combinations() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(167409079868000, sum_parts_combinations(&input));
    }

    #[test]
    fn test_sum_parts_rating() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(19114, sum_parts_rating(&input));
    }
}