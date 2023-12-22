use std::collections::{HashMap, VecDeque};

pub fn count_monkey_business(input:&str, rounds:usize, can_relax:bool) -> usize {
    let mut monkeys:Vec<Monkey> = input.split("\n\n").map(|g|Monkey::parse(g)).collect();
    let mut monkey_counts:HashMap<usize, usize> = HashMap::new();

    for _ in 0..rounds {
        do_round(&mut monkeys, &mut monkey_counts, can_relax);
    }

    let mut values = monkey_counts.values().collect::<Vec<_>>();
    values.sort();
    values.reverse();
    
    values[0] * values[1]
}

fn do_round(monkeys:&mut Vec<Monkey>, monkey_counts: &mut HashMap<usize, usize>, can_relax: bool) {
    let mut modulo = 0;
    if !can_relax {
        modulo = monkeys.into_iter().fold(1,|acc,m|lcm(acc,m.test_divisible_by));
    }
    for x in 0..monkeys.len() {
        // println!("Monkey {}:", x);
        while let Some(item) = monkeys[x].items.pop_front() {
            if let Some(count) = monkey_counts.get(&x) {
                monkey_counts.insert(x, count + 1);
            } else {
                monkey_counts.insert(x, 1);
            };
            // println!("  Monkey inspects an item with a worry level of {}.", item);
            let mut operator = item;
            if let Some(operator_num) = monkeys[x].operator {
                operator = operator_num;
            }
            let mut worry_level = match monkeys[x].operation.as_str() {
                "+" => {
                    let result = item + operator;
                    // println!("    Worry level increases by {} to {}", operator, result);
                    result
                },
                "*" => {
                    let result = item * operator;
                    // println!("    Worry level is multiplied by {} to {}", operator, result);
                    result
                },
                &_ => todo!()
            };

            if can_relax {
                worry_level /= 3;
                // println!("    Monkey gets bored with item. Worry level is divided by 3 to {}.", worry_level);
            } else {
                worry_level = worry_level.rem_euclid(modulo);
            }

            if worry_level % monkeys[x].test_divisible_by == 0 {
                let target_monkey = monkeys[x].monkey_if_true;
                // println!("    Current worry level is divisible by {}.", monkeys[x].test_divisible_by);
                // println!("    Item with worry level {} is thrown to monkey {}.", worry_level, target_monkey);
                monkeys[target_monkey].items.push_back(worry_level);
            } else {
                let target_monkey = monkeys[x].monkey_if_false;
                // println!("    Current worry level is not divisible by {}.", monkeys[x].test_divisible_by);
                // println!("    Item with worry level {} is thrown to monkey {}.", worry_level, target_monkey);
                monkeys[target_monkey].items.push_back(worry_level);
            }
        }
    }
}

fn gcd(first: usize, second: usize) -> usize
{
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

fn lcm(first: usize, second: usize) -> usize
{
    first * second / gcd(first, second)
}

struct Monkey {
    items:VecDeque<usize>,
    operation:String,
    operator:Option<usize>,
    test_divisible_by: usize,
    monkey_if_true: usize,
    monkey_if_false: usize,
}

impl Monkey {
    fn parse(input:&str) -> Monkey {
        // Monkey 0:
        // Starting items: 79, 98
        // Operation: new = old * 19
        // Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        let lines:Vec<_> = input.split("\n").collect();
        let operation_parts:Vec<_> = lines[2].split("new = old ").collect::<Vec<_>>()[1].split(" ").collect();
        let operator = match operation_parts[1] {
            "old" => None,
              _   => Some(operation_parts[1].parse::<usize>().unwrap())
        };
        Monkey{
            items: lines[1].split(": ").collect::<Vec<_>>()[1].split(", ").map(|c|c.parse::<usize>().unwrap()).collect(),
            operation: operation_parts[0].to_string(),
            operator: operator,
            test_divisible_by: lines[3].split("divisible by ").collect::<Vec<_>>()[1].parse::<usize>().unwrap(),
            monkey_if_true: lines[4].split("throw to monkey ").collect::<Vec<_>>()[1].parse::<usize>().unwrap(),
            monkey_if_false: lines[5].split("throw to monkey ").collect::<Vec<_>>()[1].parse::<usize>().unwrap(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_count_monkey_business() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file input");

        assert_eq!(10605, count_monkey_business(&input, 20, true));
    }

    #[test]
    fn test_count_insane_monkey_business() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file input");

        assert_eq!(2713310158, count_monkey_business(&input, 10000, false));
    }
}