use std::collections::VecDeque;

const CRATE_WIDTH:usize = 4;

#[derive(PartialEq)]
pub enum TowerModel {
    Tower9000,
    Tower9001
}

type TowerStack = Vec<char>;

struct Tower {
    model:TowerModel,
    stacks:Vec<TowerStack>,
}

impl Tower {
    fn move_crate(&mut self, qty:usize, from_stack:usize, to_stack:usize) {
        match self.model {
            TowerModel::Tower9000 => self.move_crate_9000(qty, from_stack, to_stack),
            TowerModel::Tower9001 => self.move_crate_9001(qty, from_stack, to_stack)
        }
    }

    fn move_crate_9000(&mut self, qty:usize, from_stack:usize, to_stack:usize) {
        for _ in 0..qty {
            if let Some(tower_crate) = self.stacks[from_stack-1].pop() {
                self.stacks[to_stack-1].push(tower_crate);
            }
        }
    }

    fn move_crate_9001(&mut self, qty:usize, from_stack:usize, to_stack:usize) {
        let from = &mut self.stacks[from_stack-1];
        let scoop = from.drain(from.len()-qty..).collect::<Vec<_>>();
        self.stacks[to_stack-1].append(&mut scoop.clone());
    }

    fn top_crates(&self) -> String {
        return String::from_iter(
            self.stacks
                .iter()
                .filter(|stack| stack.last().is_some())
                .map(|stack| *stack.last().unwrap())
                .collect::<Vec<_>>()
                .iter()
            )
            .to_string();
    }

    fn from_string(model:TowerModel, input:String) -> Self {
        let mut traversible_stacks:Vec<VecDeque<char>> = Vec::new();

        for line in input.split("\n") {
            let line_fmt = format!("{:width$}", line, width=line.len()+line.len()%CRATE_WIDTH);
            let tower_crates: Vec<_> = line_fmt.chars()
                .collect::<Vec<_>>()
                .chunks(CRATE_WIDTH)
                .collect::<Vec<_>>()
                .iter()
                .map(|c_arr| String::from_iter(c_arr.iter()).trim().to_string())
                .collect();
            for (i, tower_crate) in tower_crates.iter().cloned().enumerate() {
                while i >= traversible_stacks.len() {
                    traversible_stacks.push(VecDeque::new());
                }

                if tower_crate.len() == 3 {
                    let tower_crate_box = tower_crate.chars().collect::<Vec<_>>()[1];
                    traversible_stacks[i].push_front(tower_crate_box);
                }
            }
        }

        let tower_stacks = traversible_stacks
            .iter()
            .map(|stack| stack.iter().map(|stack_crate| *stack_crate).collect::<Vec<char>>())
            .collect::<Vec<_>>();

        return Tower{
            model: model,
            stacks: tower_stacks
        };
    }
}

pub fn figure_top_crates(model:TowerModel, input:&str) -> String {
    let crane_input = input.split("\n\n").collect::<Vec<_>>();
    let mut tower = Tower::from_string(model, crane_input[0].to_string());
    for instruction in crane_input[1].split("\n") {
        let command = instruction.split(" ").collect::<Vec<_>>();
        let qty = command[1].parse::<usize>().unwrap();
        let from_stack = command[3].parse::<usize>().unwrap();
        let to_stack = command[5].parse::<usize>().unwrap();
        tower.move_crate(qty, from_stack, to_stack);
    }
    return tower.top_crates();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tower9000_parse() {
        let tower_str = "    [D]    \n[N] [C]    \n[Z] [M] [P] \n 1   2   3 ";
        let mut tower = Tower::from_string(TowerModel::Tower9000, tower_str.to_string());
        assert_eq!(4, tower.stacks.len());
        assert_eq!("NDP", tower.top_crates());

        tower.move_crate(1, 2, 1);
        assert_eq!("DCP", tower.top_crates());

        tower.move_crate(3, 1, 3);
        assert_eq!("CZ", tower.top_crates());

        tower.move_crate(2, 2, 1);
        tower.move_crate(1, 1, 2);
        assert_eq!("CMZ", tower.top_crates());
    }

    #[test]
    fn test_tower9001_parse() {
        let tower_str = "    [D]    \n[N] [C]    \n[Z] [M] [P] \n 1   2   3 ";
        let mut tower = Tower::from_string(TowerModel::Tower9001, tower_str.to_string());
        assert_eq!(4, tower.stacks.len());
        assert_eq!("NDP", tower.top_crates());

        tower.move_crate(1, 2, 1);
        assert_eq!("DCP", tower.top_crates());

        tower.move_crate(3, 1, 3);
        assert_eq!("CD", tower.top_crates());

        tower.move_crate(2, 2, 1);
        tower.move_crate(1, 1, 2);
        assert_eq!("MCD", tower.top_crates());
    }
}