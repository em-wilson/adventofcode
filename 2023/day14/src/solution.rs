#[derive(Clone, Copy, PartialEq)]
enum Tile {
    Empty,
    CubedRock,
    RoundRock
}

impl Tile {
    fn from(input:char) -> Tile {
        match input {
            '.' => Tile::Empty,
            'O' => Tile::RoundRock,
            '#' => Tile::CubedRock,
             _  => panic!("Unknown rock {}", input)
        }
    }
}

pub fn calculate_load(input:&str, cycles:usize) -> usize {
    let mut map:Vec<Vec<Tile>> = input.split("\n")
        .map(|l|l.chars().map(|c|Tile::from(c)).collect())
        .collect();

    if cycles == 0 {
        tilt_platform_north(&mut map);
        return calculate_weight(&map);
    }

    let mut seen = vec![map.clone()];
    loop {
        for _ in 0..4 {
            tilt_platform_north(&mut map);
            map = rotate_map(&mut map);
        }
        if let Some(idx) = seen.iter().position(|x| x == &map) {
            let cycle_len = seen.len() - idx;
            let final_idx = idx + (cycles - idx) % cycle_len;
            return calculate_weight(&seen[final_idx]);
        }
        seen.push(map.clone());
    }
}

fn rotate_map(map:&Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    let size = map.len();
    let mut rotated = vec![vec![Tile::Empty; size]; size];
    for row in 0..size {
        for col in 0..size {
            rotated[col][size - 1 - row] = map[row][col];
        }
    }
    rotated
}

fn calculate_weight(map:&Vec<Vec<Tile>>) -> usize {
    let mut weight = 0;
    let nlen = map.len();
    for y in 0..nlen {
        for x in 0..map[y].len() {
            if map[y][x] == Tile::RoundRock {
                weight += nlen - y;
            }
        }
    }
    weight
}

fn tilt_platform_north(map:&mut Vec<Vec<Tile>>) {
    let nlen = map.len();
    for x in 0..map[0].len() {
        let mut landing = 0;
        let mut group_len = 0;
        let mut y = 0;
        while y < nlen {
            if map[y][x] == Tile::RoundRock {
                group_len += 1;
            }
            
            if map[y][x] == Tile::CubedRock || y == nlen - 1 {
                let y_end = match map[y][x] {
                    Tile::CubedRock => std::cmp::max(0, y as isize - 1) as usize,
                    _   => y
                };
                if y_end == landing {
                    y += 1;
                    group_len = 0;
                    landing = y;
                    continue;
                }
                for y_write in landing..=y_end {
                    if y_write - landing < group_len {
                        map[y_write][x] = Tile::RoundRock;
                    } else {
                        map[y_write][x] = Tile::Empty;
                    }
                }
                group_len = 0;
                landing = y + 1;
            }
            y += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_summarize() {
        let input = fs::read_to_string("input.txt")
            .expect("Could not read file input.txt");
        
        assert_eq!(108759, calculate_load(&input, 0));
    }

    #[test]
    fn test_calculate_load() {
        assert_eq!(136, calculate_load("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", 0));
        assert_eq!(64, calculate_load("O....#....\nO.OO#....#\n.....##...\nOO.#O....O\n.O.....O#.\nO.#..O.#.#\n..O..#O..O\n.......O..\n#....###..\n#OO..#....", 1000000000));
    }
}