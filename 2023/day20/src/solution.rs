use std::collections::{HashMap, VecDeque};

pub fn pulse_product(input:&str) -> usize {
    let mut modules = parse_input(input);

    let mut pulse_count_list = vec!();

    for _ in 1..=1000 {
        let mut pulse_counts = (0,0);

        let mut pq = VecDeque::new();
        pq.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((sender, receiver, pulse)) = pq.pop_front() {
            match pulse {
                Pulse::Low => pulse_counts.0 += 1,
                Pulse::High => pulse_counts.1 += 1
            }
            if let Some(module) = modules.get_mut(&receiver) {
                match module.module_type {
                    ModuleType::Broadcaster => {
                        for destination in module.destinations.iter() {
                            pq.push_back((module.name.to_string(), destination.to_string(), pulse.clone()));
                        }
                    },
                    ModuleType::Conjunction => {
                        module.connectors.insert(sender.to_string(), pulse);
                        let next_pulse = match module.connectors.values().all(|i|*i == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High
                        };
                        for destination in module.destinations.iter() {
                            pq.push_back((module.name.to_string(), destination.to_string(), next_pulse));
                        }
                    },
                    ModuleType::FlipFlop => {
                        match pulse {
                            Pulse::Low => {
                                module.flip_state();
                                for destination in module.destinations.iter() {
                                    pq.push_back((module.name.to_string(), destination.to_string(), module.state));
                                }
                            },
                            Pulse::High => ()
                        }
                    }
                }
            }
        }
        pulse_count_list.push(pulse_counts);
    }
    
    let pulse_counts = pulse_count_list.iter().fold((0,0),|acc,pc|(acc.0 + pc.0, acc.1 + pc.1));
    pulse_counts.0 * pulse_counts.1
}

pub fn trigger_rx(input:&str) -> usize {
    let mut modules = parse_input(input);

    // Tightly couples the code to my particular input... but there are 4 places that activate rg which in turn triggers rx
    let main_nodes:Vec<_> = modules.values().filter(|m|m.module_type == ModuleType::Conjunction && m.destinations[0] == "rg")
        .map(|m|m.name.clone())
        .collect();

    let mut min_presses:HashMap<String, usize> = HashMap::new();

    let mut presses = 0;
    loop {
        presses += 1;

        let mut pq = VecDeque::new();
        pq.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((sender, receiver, pulse)) = pq.pop_front() {
            if receiver == "rx" && pulse == Pulse::Low {
                return presses;
            }
            
            if let Some(module) = modules.get_mut(&receiver) {
                match module.module_type {
                    ModuleType::Broadcaster => {
                        for destination in module.destinations.iter() {
                            pq.push_back((module.name.to_string(), destination.to_string(), pulse.clone()));
                        }
                    },
                    ModuleType::Conjunction => {
                        module.connectors.insert(sender.to_string(), pulse);
                        let next_pulse = match module.connectors.values().all(|i|*i == Pulse::High) {
                            true => Pulse::Low,
                            false => Pulse::High
                        };
                        for destination in module.destinations.iter() {
                            pq.push_back((module.name.to_string(), destination.to_string(), next_pulse));
                        }

                        if main_nodes.contains(&module.name) && next_pulse == Pulse::High {
                            if !min_presses.contains_key(&module.name) {
                                min_presses.insert(module.name.to_string(), presses);
                            }
                            if min_presses.len() == main_nodes.len() {
                                let mut values:Vec<usize> = min_presses.values().cloned().collect();
                                while values.len() > 1 {
                                    values = values.chunks(2).map(|vals|lcm(vals[0],vals[1])).collect();
                                }
                                return values[0];
                            }
                        }
            
                    },
                    ModuleType::FlipFlop => {
                        match pulse {
                            Pulse::Low => {
                                module.flip_state();
                                for destination in module.destinations.iter() {
                                    pq.push_back((module.name.to_string(), destination.to_string(), module.state));
                                }
                            },
                            Pulse::High => ()
                        }
                    }
                }
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

fn parse_input(input:&str) -> HashMap<String, Module> {
    let mut modules:HashMap<String, Module> = input.split("\n")
    .map(|l|{let m = Module::parse(l);(m.name.to_string(),m)})
    .collect();

    // Hook up conjunctions
    for module in modules.clone().values() {
        match module.module_type {
            ModuleType::FlipFlop => {
                for destination in module.destinations.iter() {
                    let strr = destination.clone();
                    if let Some(dest_module) = modules.get_mut(&strr) {
                        match dest_module.module_type {
                            ModuleType::Conjunction => {
                                dest_module.connectors.insert(module.name.clone(), Pulse::Low);
                            },
                            _ => ()
                        }
                    }
                }
            },
            _ => ()
        }
    }
    modules
}

#[derive(Clone,Copy,Debug,Eq,PartialEq)]
enum Pulse {
    Low,
    High
}

impl Pulse {
    fn flip(&self) -> Pulse {
        match self {
            Pulse::High => Pulse::Low,
            Pulse::Low => Pulse::High
        }
    }
}

#[derive(Clone)]
struct Module {
    name:String,
    state:Pulse,
    module_type:ModuleType,
    destinations:Vec<String>,
    connectors:HashMap<String, Pulse>
}

impl Module {
    fn parse(line:&str) -> Module {
        let mut part_type = ModuleType::Broadcaster;
        let parts:Vec<_> = line.split(" -> ").collect();
        let mut name = parts[0].chars();
        match name.next() {
            Some('%') => part_type = ModuleType::FlipFlop,
            Some('&') => part_type = ModuleType::Conjunction,
             _  => name = parts[0].chars()
        };
        Module {
            name:name.collect(),
            state:Pulse::Low,
            module_type: part_type,
            destinations: parts[1].split(", ").map(|s|s.to_string()).collect(),
            connectors: HashMap::new(),
        }
    }

    fn flip_state(&mut self) {
        self.state = self.state.flip();
    }
}

#[derive(Clone, Eq, PartialEq)]
enum ModuleType {
    Broadcaster,
    FlipFlop,
    Conjunction
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_pulse_product() {
        let input_a = fs::read_to_string("test_input_a.txt")
            .expect("Could not read file test_input.txt");
        let input_b = fs::read_to_string("test_input_b.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(32000000, pulse_product(&input_a));
        assert_eq!(11687500, pulse_product(&input_b));
    }
}