use std::collections::HashSet;

pub fn count_garden_plots(input:&str, step_count:usize) -> usize {
    let map:GardenMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let (x,y) = get_start_pos(&map);

    let mut frontier:HashSet<(usize,usize)> = HashSet::new();
    frontier.insert((x, y));
    for _ in 0..step_count {
        let mut next:HashSet<(usize,usize)> = HashSet::new();
        for (x, y) in frontier.drain() {
            if x > 0 && map[y][x - 1] != '#' {
                next.insert((x - 1, y));
            }
            if x < map[y].len() - 1 && map[y][x + 1] != '#' {
                next.insert((x + 1, y));
            }
            if y > 0 && map[y - 1][x] != '#' {
                next.insert((x, y - 1));
            }
            if y < map.len() - 1 && map[y + 1][x] != '#' {
                next.insert((x, y + 1));
            }
        }
        frontier = next.clone();
    }
    frontier.len()
}

pub fn count_infinite_garden_plots(input:&str, step_count:usize) -> usize {
    let map:GardenMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let (x,y) = get_start_pos(&map);

    let mut frontier:HashSet<(i64,i64)> = HashSet::new();
    frontier.insert((x as i64, y as i64));
    let mut coefficients:Vec<usize> = Vec::new();
    let map_area = map.len();
    let step_remainder = step_count.rem_euclid(map_area);
    // println!("Step remainder: {} % {} = {}", step_count, map_area, step_remainder);

    let mut remainder = 0;
    for step_num in 0..step_count {
        if remainder == map_area {
            remainder = 0;
        }
        let mut next:HashSet<(i64,i64)> = HashSet::new();
        let prev_visited = frontier.len();
        for (x, y) in frontier.drain() {
            let mod_y = y.rem_euclid(map.len() as i64) as usize;
            let mod_x = x.rem_euclid(map[mod_y].len() as i64) as usize;
            let mod_up = (y - 1).rem_euclid(map.len() as i64) as usize;
            let mod_down = (y + 1).rem_euclid(map.len() as i64) as usize;
            let mod_left = (x - 1).rem_euclid(map[mod_y].len() as i64) as usize;
            let mod_right = (x + 1).rem_euclid(map[mod_y].len() as i64) as usize;

            if map[mod_y][mod_left] != '#' {
                next.insert((x - 1, y));
            }

            if map[mod_y][mod_right] != '#' {
                next.insert((x + 1, y));
            }

            if map[mod_up][mod_x] != '#' {
                next.insert((x, y - 1));
            }
            if map[mod_down][mod_x] != '#' {
                next.insert((x, y + 1));
            }
        }

        if remainder == step_remainder {
            // println!("Step {}", step_num);
            coefficients.push(prev_visited);
            if coefficients.len() == 3 {
                // println!("Coefficients {} {} {}", coefficients[0], coefficients[1], coefficients[2]);
                let x = step_count / map_area;
                let b0 = coefficients[0];
                let b1 = coefficients[1] - coefficients[0];
                let b2 = coefficients[2] - coefficients[1];

                // println!("step remainder {}", x);

                return b0 + (b1 * x) + ( (x * ( x - 1 ) / 2 ) * (b2 - b1) );
            }
        }
        frontier = next.clone();
        remainder += 1;
    }
    frontier.len()
}

type GardenMap = Vec<Vec<char>>;

fn get_start_pos(map:&GardenMap) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return (x,y);
            }
        }
    }
    panic!("No start location found");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count_garden_plots() {
        assert_eq!(16, count_garden_plots("...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........", 6));
    }

    #[test]
    fn test_count_infinite_garden_plots() {
        assert_eq!(16, count_infinite_garden_plots("...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........", 6));
        assert_eq!(50, count_infinite_garden_plots("...........\n.....###.#.\n.###.##..#.\n..#.#...#..\n....#.#....\n.##..S####.\n.##..#...#.\n.......##..\n.##.#.####.\n.##..##.##.\n...........", 10));
    }
}