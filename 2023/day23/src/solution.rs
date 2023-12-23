use std::collections::HashSet;

pub fn longest_path(input:&str, can_navigate_slopes:bool) -> usize {
    let map:ForestMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let paths = map.find_paths(can_navigate_slopes);
    *paths.iter().max().unwrap()
}

type Pos = (usize, usize);
type ForestMap = Vec<Vec<char>>;

trait TrailHeads {
    fn find_exits(&self) -> (Pos, Pos);
}

impl TrailHeads for ForestMap {
    fn find_exits(&self) -> (Pos, Pos) {
        let mut start_pos = (0,0);
        let mut end_pos = start_pos;
        let final_row = self.len() - 1;
        for x in 0..self[0].len() {
            if self[0][x] == '.' {
                start_pos = (x,0);
            }
            if self[final_row][x] == '.' {
                end_pos = (x,final_row);
            }
        }
        (start_pos, end_pos)
    }
}

trait PathFinder: TrailHeads {
    fn find_paths(&self, can_navigate_slopes:bool) -> Vec<usize>;
}

impl PathFinder for ForestMap {
    fn find_paths(&self, can_navigate_slopes:bool) -> Vec<usize> {
        let (start_pos, end_pos) = self.find_exits();

        let mut steps:Vec<Step> = Vec::new();
        let mut paths:Vec<usize> = Vec::new();

        steps.push(Step {
            visited:HashSet::new(),
            count:0,
            pos:start_pos,
        });

        while let Some(mut step) = steps.pop() {
            if step.pos == end_pos {
                paths.push(step.count);
            } else {
                step.visited.insert(step.pos);

                for neighbour in step.valid_moves(self, can_navigate_slopes) {
                    steps.push({
                        Step {
                            visited:step.visited.clone(),
                            count:step.count + 1,
                            pos: neighbour
                        }
                    })
                }
            }
        }

        paths
    }
}

struct Step {
    visited:HashSet<Pos>,
    count:usize,
    pos:Pos,
}

impl Step {
    fn valid_moves(&self, map:&ForestMap, can_navigate_slopes:bool) -> Vec<Pos> {
        let moves = match (can_navigate_slopes, map[self.pos.1][self.pos.0]) {
            (false, '>') => vec!((self.pos.0 + 1, self.pos.1)),
            (false, '<') => vec!((self.pos.0 - 1, self.pos.1)),
            (false, '^') => vec!((self.pos.0, self.pos.1 - 1)),
            (false, 'v') => vec!((self.pos.0, self.pos.1 + 1)),
            _   => {
                let mut results:Vec<Pos> = Vec::new();
                if self.pos.0 > 0 {
                    results.push((self.pos.0 - 1, self.pos.1));
                }
                if self.pos.0 < map[self.pos.1].len() - 1 {
                    results.push((self.pos.0 + 1, self.pos.1));
                }
                if self.pos.1 > 0 {
                    results.push((self.pos.0, self.pos.1 -  1));
                }
                if self.pos.1 < map.len() - 1 {
                    results.push((self.pos.0, self.pos.1 + 1));
                }
                results
            }
        };

        moves.into_iter().filter(|m|!self.visited.contains(m) && map[m.1][m.0] != '#').collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_longest_path_with_slippery_slopes() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(94, longest_path(&input, false));
    }

    #[test]
    fn test_longest_path_with_good_boots() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(154, longest_path(&input, true));
    }
}