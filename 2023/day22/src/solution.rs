use std::collections::{HashSet,HashMap};
use std::collections::VecDeque;

pub fn dissolve_safe_bricks(input:&str) -> usize {
    let mut to_process:Vec<Brick> = input.split("\n")
        .enumerate()
        .map(|(id,line)|Brick::parse(id,line))
        .collect();

    to_process.sort_by(|b, a|a.front.2.cmp(&b.back.2));

    let mut processed:Vec<Brick> = vec!();
    let mut supported_by:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut supporting:HashMap<usize,Vec<usize>> = HashMap::new();
    while let Some(curr_brick) = to_process.pop() {
        let overlapping:Vec<&Brick> = processed
            .iter()
            .filter(|brick| brick.overlaps(&curr_brick))
            .collect();
        let highest_z:usize = overlapping
            .iter()
            .map(|brick|brick.front.2)
            .max()
            .unwrap_or(0);
        let bricks_to_fall_onto:Vec<usize> = overlapping
            .iter()
            .filter_map(|brick|{
                if brick.back.2 == highest_z {
                    Some(brick.id)
                } else {
                    None
                }
            })
            .collect();

        if bricks_to_fall_onto.len() > 0 {
            supported_by.insert(curr_brick.id, bricks_to_fall_onto.clone());
            for brick_id in bricks_to_fall_onto {
                supporting.entry(brick_id).or_default().push(curr_brick.id);
            }
        }
        let drop = curr_brick.front.2 - (highest_z + 1);
        let mut new_front = curr_brick.front;
        new_front.2 = new_front.2 - drop;
        let mut new_back = curr_brick.back;
        new_back.2 = new_back.2 - drop;
        processed.push(Brick {
            id:curr_brick.id,
            front:new_front,
            back:new_back
        });
    }

    let mut foundations:HashSet<usize> = HashSet::new();
    for (_, supports) in supported_by {
        if supports.len() == 1 {
            foundations.insert(supports[0]);
        }
    }

    // 471 is correct
    processed.len() - foundations.len()
}

fn tetris_bricks(input:&str, processed:&mut Vec<Brick>, supported_by:&mut HashMap<usize,Vec<usize>>, supporting:&mut HashMap<usize,Vec<usize>>) {
    let mut to_process:Vec<Brick> = input.split("\n")
        .enumerate()
        .map(|(id,line)|Brick::parse(id,line))
        .collect();

    to_process.sort_by(|b, a|a.front.2.cmp(&b.back.2));
    for brick in &to_process {
        supporting.insert(brick.id, Vec::new());
    }

    while let Some(curr_brick) = to_process.pop() {
        let overlapping:Vec<&Brick> = processed
            .iter()
            .filter(|brick| brick.overlaps(&curr_brick))
            .collect();
        let highest_z:usize = overlapping
            .iter()
            .map(|brick|std::cmp::max(brick.front.2,brick.back.2))
            .max()
            .unwrap_or(0);
        let bricks_to_fall_onto:Vec<usize> = overlapping
            .iter()
            .filter_map(|brick|{
                if std::cmp::max(brick.front.2,brick.back.2) == highest_z {
                    Some(brick.id)
                } else {
                    None
                }
            })
            .collect();

        if bricks_to_fall_onto.len() > 0 {
            supported_by.insert(curr_brick.id, bricks_to_fall_onto.clone());
            for brick_id in bricks_to_fall_onto {
                supporting.entry(brick_id).or_default().push(curr_brick.id);
            }
        }
        let drop = curr_brick.front.2 - (highest_z + 1);
        let mut new_front = curr_brick.front;
        new_front.2 = new_front.2 - drop;
        let mut new_back = curr_brick.back;
        new_back.2 = new_back.2 - drop;
        processed.push(Brick {
            id:curr_brick.id,
            front:new_front,
            back:new_back
        });
    }
}

pub fn count_falling_bricks(input:&str) -> usize {
    let mut processed:Vec<Brick> = vec!();
    let mut supported_by:HashMap<usize,Vec<usize>> = HashMap::new();
    let mut supporting:HashMap<usize,Vec<usize>> = HashMap::new();
    tetris_bricks(input, &mut processed, &mut supported_by, &mut supporting);

    let mut foundations:HashSet<usize> = HashSet::new();
    for supports in supported_by.values() {
        if supports.len() == 1 {
            foundations.insert(supports[0]);
        }
    }

    let mut felled_bricks:usize = 0;
    // for supporting_brick in foundations {
    //     felled_bricks += count_felled_bricks(supporting_brick, &supporting, &supported_by);
    // }

    for brick in processed {
        felled_bricks += count_felled_bricks(brick.id, &supporting, &supported_by);
    }

    // 68519 is too low
    // 68519 is too low
    felled_bricks
}

fn count_felled_bricks(brick:usize, supporting:&HashMap<usize, Vec<usize>>, supported_by:&HashMap<usize, Vec<usize>>) -> usize {
    let mut queue: VecDeque<usize> = supporting[&brick]
        .iter()
        .filter(|b| supported_by[b].len() == 1)
        .cloned()
        .collect();
    let mut falling: HashSet<usize> = queue.iter().cloned().collect();
    falling.insert(brick);

    while let Some(top_brick) = queue.pop_front() {
        for supported in &supporting[&top_brick] {
            if falling.contains(supported) {
                continue;
            }
            let mut already_falling = 0;
            for supporting in &supported_by[supported] {
                if falling.contains(supporting) {
                    already_falling += 1;
                }
            }
            if already_falling == supported_by[supported].len() {
                falling.insert(*supported);
                queue.push_back(*supported);
            }
        }
    }

    falling.len() - 1
}

#[derive(Clone)]
struct Brick {
    id:usize,
    front:(usize,usize,usize),
    back:(usize,usize,usize),
}

impl Brick {
    fn parse(id:usize, input:&str) -> Brick {
        let parts:Vec<(usize, usize, usize)> = input.split("~").map(|p|{
            let coords:Vec<usize> = p.split(",").map(|c|c.parse::<usize>().unwrap()).collect();
            (coords[0], coords[1], coords[2])
        }).collect();
        
        Brick{
            id:id,
            front:parts[0],
            back:parts[1]
        }
    }

    fn overlaps(&self, brick:&Brick) -> bool {
        self.front.0 <= brick.back.0 && self.back.0 >= brick.front.0
            && self.front.1 <= brick.back.1 && self.back.1 >= brick.front.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_real_dissolve_bricks() {
        let input = fs::read_to_string("input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(471, dissolve_safe_bricks(&input));
    }
    
    #[test]
    fn test_dissolve_bricks() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(5, dissolve_safe_bricks(&input));
    }
    
    #[test]
    fn test_real_count_falling_bricks() {
        let input = fs::read_to_string("input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(68525, count_falling_bricks(&input));
    }
    #[test]
    fn test_count_falling_bricks() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(7, count_falling_bricks(&input));
    }
}