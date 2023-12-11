type PipeMap = Vec<Vec<char>>;

pub fn count_furthest_step(input:&str) -> usize {
    let map = parse_map(input);
    let start_pos = get_start_pos(&map);
    let pipe_len = get_pipe_length(&map, start_pos, None);

    match pipe_len % 2 {
        0 => pipe_len / 2,
        1 => (pipe_len + 1) / 2,
        _ => panic!("invalid pipe length")
    }
}

pub fn count_enclosed_tiles(input:&str) -> usize {
    let map = parse_map(input);
    let mut bitmap:Vec<Option<bool>> = vec![None; map.len() * map[0].len()];
    let start_pos = get_start_pos(&map);
    get_pipe_length(&map, start_pos, Some(&mut bitmap));

    let mut scaled_map = upscale_map(&map, &bitmap);
    flood_fill(&mut scaled_map, (0,0));
    let rescaled_map = downscale_map(&mut scaled_map);
    rescaled_map.into_iter().flat_map(|l|l).filter(|i|i == &'G').count()
}

fn downscale_map(map: &mut PipeMap) -> PipeMap {
    let mut scaled_map = vec![vec!['G'; map[0].len()/3]; map.len()/3];

    for y in 0..map.len() / 3 {
        for x in 0..map[y].len() / 3 {
            scaled_map[y][x] = map[(y * 3) + 1][(x * 3) + 1];
        }
    }
    scaled_map
}

fn upscale_map(map:&PipeMap, bitmap:&Vec<Option<bool>>) -> PipeMap {
    let mut scaled_map = vec![vec!['G'; map[0].len()*3]; map.len()*3];
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let idx = y * map[0].len() + x;
            if let Some(has_pipe) = bitmap[idx] {
                let scaled_y = y * 3;
                let scaled_x = x * 3;

                if has_pipe {
                    let map_element = match map[y][x] {
                        'S' => uncover_pipe_at(map, (x,y)),
                         _  => map[y][x]
                    };
                    match map_element {
                        '|' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'P';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'G';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'G';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'P';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        '-' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'G';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'P';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'P';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'G';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        'F' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'G';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'G';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'P';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'P';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        '7' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'G';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'P';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'G';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'P';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        'L' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'P';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'G';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'P';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'G';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        'J' => {
                            scaled_map[scaled_y][scaled_x+0] = 'G';
                            scaled_map[scaled_y][scaled_x+1] = 'P';
                            scaled_map[scaled_y][scaled_x+2] = 'G';
                            scaled_map[scaled_y+1][scaled_x+0] = 'P';
                            scaled_map[scaled_y+1][scaled_x+1] = 'P';
                            scaled_map[scaled_y+1][scaled_x+2] = 'G';
                            scaled_map[scaled_y+2][scaled_x+0] = 'G';
                            scaled_map[scaled_y+2][scaled_x+1] = 'G';
                            scaled_map[scaled_y+2][scaled_x+2] = 'G';
                        },
                        '.' => { }
                        _ => panic!("no match for {}", map[y][x])
                    }
                }                
            }

        }
    }
    scaled_map
}

fn uncover_pipe_at(map:&PipeMap, pos:(usize, usize)) -> char {
    // 8421
    // NESW
    let mut open = 0b0000;
    let (x,y) = pos;

    if y > 0 && ['F','|','7'].contains(&map[y-1][x]) {
        open |= 0b1000;
    }
    if x < map[0].len() - 1 && ['7', '-', 'J'].contains(&map[y][x+1]) {
        open |= 0b0100;
    }
    if y < map.len() - 1 && ['L', '|', 'J'].contains(&map[y+1][x]) {
        open |= 0b0010;
    }
    if x > 0 && ['F', '-', 'L'].contains(&map[y][x-1]) {
        open |= 0b0001;
    }

    match open {
        0b1001 => 'J',
        0b1010 => '|',
        0b1100 => 'L',
        0b0101 => '-',
        0b0110 => 'F',
        0b0011 => 'J',
        _       => panic!("unknown open dir {}", open)
    }
}

fn flood_fill(map:&mut PipeMap, pos:(usize, usize)) {
    let (x, y) = pos;
    if map[y][x] == 'G' {
        map[y][x] = 'O';
        if x < map[y].len() - 1 {
            flood_fill(map, (x + 1, y));
        }
        if x > 0 {
            flood_fill(map, (x - 1, y));
        }
        if y < map.len() - 1 {
            flood_fill(map, (x, y + 1));
        }
        if y > 0 {
            flood_fill(map, (x, y - 1));
        }
    }
}

fn parse_map(input:&str) -> PipeMap {
    input.split("\n")
        .map(|line|line.chars().collect())
        .collect()
}

fn get_pipe_length(map:&PipeMap, start_pos:(usize, usize), mut bit_map:Option<&mut Vec<Option<bool>>>) -> usize {
    let mut step_count = 1;
    let (x1, y1) = get_first_step(&map, start_pos);
    let (start_x, start_y) = start_pos;
    let start_pos = (start_x as isize, start_y as isize);
    if let Some(bitmap) = bit_map.as_mut() {
        bitmap[start_y as usize * map[0].len() + start_x as usize] = Some(true);
    }

    let mut next_step = (x1 as isize, y1 as isize);
    let mut prev_step = start_pos;

    while next_step != start_pos {
        let (x, y) = next_step;
        if let Some(bitmap) = bit_map.as_mut() {
            bitmap[y as usize * map[0].len() + x as usize] = Some(true);
        }
        step_count += 1;
        let h = next_step;
        next_step = get_next_step(&map, prev_step, next_step);
        prev_step = h;
    }
    step_count
}

fn get_next_step(map:&PipeMap, prev_pos:(isize, isize), curr_pos:(isize, isize)) -> (isize, isize) {
    let (x1, y1) = prev_pos;
    let (x2, y2) = curr_pos;

    match map[y2 as usize][x2 as usize] {
        '|' => (x2, y2 + (y2 - y1)),
        '-' => (x2 + (x2 -x1), y2),
        'L' => (if x1 == x2 { x2 + 1 } else { x2 }, if y1 == y2 { y2 - 1 } else { y2 }),
        'J' => (if x1 == x2 { x2 - 1 } else { x2 }, if y1 == y2 { y2 - 1 } else { y2 }),
        '7' => (if x1 == x2 { x2 - 1 } else { x2 }, if y1 == y2 { y2 + 1 } else { y2 }),
        'F' => (if x1 == x2 { x2 + 1 } else { x2 }, if y1 == y2 { y2 + 1 } else { y2 }),
        _ => panic!("can't handle {}", map[y2 as usize][x2 as usize])
    }
}

fn get_first_step(map:&PipeMap, start_pos:(usize, usize)) -> (usize, usize) {
    let (x, y) = start_pos;
    if x > 0 && (map[y][x-1] == '-' || map[y][x-1] == 'L' || map[y][x-1] == 'F') {
        return (x-1, y);
    }

    if x < map[y].len() - 1 && (map[y][x+1] == '-' || map[y][x+1] == 'J' || map[y][x+1] == '7') {
        return (x+1, y);
    }

    if y > 0 && (map[y-1][x] == '|' || map[y-1][x] == '7' || map[y-1][x] == 'F') {
        return (x, y-1);
    }

    if y < map.len() && (map[y+1][x] == '|' || map[y+1][x] == 'L' || map[y+1][x] == 'J') {
        return (x, y+1);
    }

    panic!("can't figure first step");
}

fn get_start_pos(map:&PipeMap) -> (usize, usize) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            if map[y][x] == 'S' {
                return (x, y);
            }
        }
    }
    panic!("can't find start pos");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_furthest_step() {
        assert_eq!(4, count_furthest_step(".....\n.S-7.\n.|.|.\n.L-J.\n....."));
        assert_eq!(8, count_furthest_step("..F7.\n.FJ|.\nSJ.L7\n|F--J\nLJ..."));
    }

    #[test]
    fn test_count_enclosed_tiles() {
        assert_eq!(4, count_enclosed_tiles("...........\n.S-------7.\n.|F-----7|.\n.||.....||.\n.||.....||.\n.|L-7.F-J|.\n.|..|.|..|.\n.L--J.L--J.\n..........."));
        assert_eq!(8, count_enclosed_tiles(".F----7F7F7F7F-7....\n.|F--7||||||||FJ....\n.||.FJ||||||||L7....\nFJL7L7LJLJ||LJ.L-7..\nL--J.L7...LJS7F-7L7.\n....F-J..F7FJ|L7L7L7\n....L7.F7||L7|.L7L7|\n.....|FJLJ|FJ|F7|.LJ\n....FJL-7.||.||||...\n....L---J.LJ.LJLJ..."))
    }
}