use std::collections::{HashMap, HashSet};

pub fn longest_path(input:&str, slippery:bool) -> usize {
    let map:ForestMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let paths = map.find_paths(slippery);
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
    fn find_paths(&self, slippery:bool) -> Vec<usize>;
    fn condense_paths(&self, start:Pos, slippery:bool) -> HashMap<Pos, Node>;
}

impl PathFinder for ForestMap {
    fn find_paths(&self, slippery:bool) -> Vec<usize> {
        let (start_pos, end_pos) = self.find_exits();

        let nodes = self.condense_paths(start_pos, slippery);

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
            } else if step.visited.insert(step.pos) {
                if let Some(node) = nodes.get(&step.pos) {
                    for (neighbour, cost) in &node.paths {
                        steps.push({
                            Step {
                                visited:step.visited.clone(),
                                count:step.count + cost,
                                pos: neighbour.clone()
                            }
                        })
                    }
                }
            }
        }

        paths
    }

    fn condense_paths(&self, start:Pos, slippery:bool) -> HashMap<Pos, Node> {
        let mut nodes:HashMap<Pos, Node> = HashMap::new();
        let mut to_process:Vec<Pos> = Vec::new();
        to_process.push(start);
        while let Some(pos) = to_process.pop() {
            if !nodes.contains_key(&pos) {
                let node = Node::from(pos, self, &nodes.keys().cloned().collect(), slippery);
                for (next, _) in &node.paths {
                    to_process.push(*next);
                }
                nodes.insert(pos, node);
            }
        }
        nodes
    }
}

struct Step {
    visited:HashSet<Pos>,
    count:usize,
    pos:Pos,
}

struct Node {
    paths:HashSet<(Pos,usize)>
}

impl Node {
    fn from(start:Pos, map:&ForestMap, _visited:&HashSet<Pos>, slippery: bool) -> Node {
        let search_paths = valid_moves(start, map, slippery);
        let mut node_paths = HashSet::new();
        for pos in search_paths {
            let mut cost = 1;
            let mut valid_exits:Vec<Pos> = valid_moves(pos, map, slippery)
                .into_iter()
                .filter(|&p|p != start)
                .collect();
            let mut destination = pos;
            while valid_exits.len() == 1 {
                cost += 1;
                let curr = destination;
                destination = valid_exits[0];
                valid_exits = valid_moves(destination, map, slippery)
                    .into_iter()
                    .filter(|&p|p != curr)
                    .collect();
            }

            node_paths.insert((destination, cost));
        }
        Node {
            paths: node_paths
        }
    }
}

fn valid_moves(pos:Pos, map:&ForestMap, slippery:bool) -> Vec<Pos> {
    let moves = match (slippery, map[pos.1][pos.0]) {
        (true, '>') => vec!((pos.0 + 1, pos.1)),
        (true, '<') => vec!((pos.0 - 1, pos.1)),
        (true, '^') => vec!((pos.0, pos.1 - 1)),
        (true, 'v') => vec!((pos.0, pos.1 + 1)),
        _   => {
            let mut results:Vec<Pos> = Vec::new();
            if pos.0 > 0 {
                results.push((pos.0 - 1, pos.1));
            }
            if pos.0 < map[pos.1].len() - 1 {
                results.push((pos.0 + 1, pos.1));
            }
            if pos.1 > 0 {
                results.push((pos.0, pos.1 -  1));
            }
            if pos.1 < map.len() - 1 {
                results.push((pos.0, pos.1 + 1));
            }
            results
        }
    };

    moves.into_iter().filter(|m|*m != pos && map[m.1][m.0] != '#').collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_longest_path_with_slippery_slopes() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(94, longest_path(&input, true));
    }

    #[test]
    fn test_real_longest_path_with_slippery_slopes() {
        let input = fs::read_to_string("input.txt")
            .expect("Could not read file input.txt");

        assert_eq!(2334, longest_path(&input, true));
    }

    #[test]
    fn test_longest_path_with_good_boots() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(154, longest_path(&input, false));
    }
}