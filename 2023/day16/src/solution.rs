use std::collections::HashSet;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
enum BeamDirection {
    North,
    East,
    West,
    South
}
type BeamPosition = (usize, usize); // x, y
type LightBeam = (BeamDirection, BeamPosition);
type MirrorMap = Vec<Vec<char>>;

fn _count_max_energized_tiles(input:&str) -> usize {
    let map:MirrorMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let mut max_count = 0;

    for y in 0..map.len() {
        max_count = std::cmp::max(max_count, count_energized_tiles_from(&map, create_bean(BeamDirection::East, (0, y))));
        max_count = std::cmp::max(max_count, count_energized_tiles_from(&map, create_bean(BeamDirection::West, (map.len() - 1 , y))));
    }

    for x in 0..map.len() {
        max_count = std::cmp::max(max_count, count_energized_tiles_from(&map, create_bean(BeamDirection::South, (x, 0))));
        max_count = std::cmp::max(max_count, count_energized_tiles_from(&map, create_bean(BeamDirection::North, (x, map[0].len() - 1))));
    }

    max_count
}

pub fn par_count_max_energized_tiles(input:&str) -> usize {
    let map:MirrorMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    let mut starting_beams = vec!();

    for y in 0..map.len() {
        starting_beams.push(create_bean(BeamDirection::East, (0, y)));
        starting_beams.push(create_bean(BeamDirection::West, (map.len() - 1 , y)));
    }

    for x in 0..map.len() {
        starting_beams.push(create_bean(BeamDirection::South, (x, 0)));
        starting_beams.push(create_bean(BeamDirection::North, (x, map[0].len() - 1)));
    }

    starting_beams.par_iter()
        .map(|beam|count_energized_tiles_from(&map, *beam))
        .max().unwrap()
}

pub fn count_energized_tiles(input:&str) -> usize {
    let map:MirrorMap = input.split("\n")
        .map(|l|l.chars().collect())
        .collect();

    count_energized_tiles_from(&map, create_bean(BeamDirection::East, (0, 0)))
}

fn count_energized_tiles_from(map:&MirrorMap, initial_beam:LightBeam) -> usize {
    let mut beams = vec!(initial_beam);
    let mut energized:HashSet<(usize, usize)> = HashSet::new();
    let mut previous_beams:HashSet<LightBeam> = HashSet::new();

    while beams.len() > 0 {
        let mut beam_idx = 0;
        while beam_idx < beams.len() {
            if previous_beams.contains(&beams[beam_idx]) {
                beams.remove(beam_idx);
                continue;
            }
            previous_beams.insert(beams[beam_idx]);
            let (dir, (x, y)) = beams[beam_idx];
            if y >= map.len() || x >= map[y].len() {
                // println!("removing overflow");
                beams.remove(beam_idx);
                continue;
            }
            energized.insert((x, y));
            // println!("{} - {:?} : ({}, {})", beam_idx,dir, x, y);
            match dir {
                BeamDirection::North => {
                    match map[y][x] {
                        '-' => {
                            beams.remove(beam_idx);
                            beams.push(create_bean(BeamDirection::East, (x+1, y)));
                            if x > 0 {
                                beams.push(create_bean(BeamDirection::West, (x-1, y)));
                            }
                        },
                        '\\' => {
                            if x > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::West, (x-1, y));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        },
                        '/' => {
                            beams[beam_idx] = create_bean(BeamDirection::East, (x+1, y));
                        },
                        _ => {
                            if y > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::North, (x, y-1));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        }
                    }
                },

                BeamDirection::East => {
                    match map[y][x] {
                        '|' => {
                            beams.remove(beam_idx);
                            if y > 0 {
                                beams.push(create_bean(BeamDirection::North, (x, y-1)));
                            }
                            beams.push(create_bean(BeamDirection::South, (x, y+1)));
                        },
                        '\\' => {
                            beams[beam_idx] = create_bean(BeamDirection::South, (x, y+1));
                        },
                        '/' => {
                            if y > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::North, (x, y - 1));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        },
                        _ => {
                            beams[beam_idx] = create_bean(BeamDirection::East, (x+1, y));
                        }
                    }
                },

                BeamDirection::West => {
                    match map[y][x] {
                        '|' => {
                            beams.remove(beam_idx);
                            if y > 0 {
                                beams.push(create_bean(BeamDirection::North, (x, y-1)));
                            }
                            beams.push(create_bean(BeamDirection::South, (x, y+1)));
                        },
                        '\\' => {
                            if y > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::North, (x, y-1));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        },
                        '/' => {
                            beams[beam_idx] = create_bean(BeamDirection::South, (x, y+1));
                        },
                        _ => {
                            if x > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::West, (x-1, y));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        }
                    }
                },

                BeamDirection::South => {
                    match map[y][x] {
                        '-' => {
                            beams.remove(beam_idx);
                            beams.push(create_bean(BeamDirection::East, (x+1, y)));
                            if x > 0 {
                                beams.push(create_bean(BeamDirection::West, (x-1, y)));
                            }
                        },
                        '\\' => {
                            beams[beam_idx] = create_bean(BeamDirection::East, (x+1, y));
                        },
                        '/' => {
                            if x > 0 {
                                beams[beam_idx] = create_bean(BeamDirection::West, (x-1, y));
                            } else {
                                beams.remove(beam_idx);
                                continue;
                            }
                        },
                        _ => {
                            beams[beam_idx] = create_bean(BeamDirection::South, (x, y+1));
                        }
                    }
                }
            }
            beam_idx += 1;
        }
    }

    energized.len()
}

fn create_bean(direction:BeamDirection, pos:(usize,usize)) -> LightBeam {
    (direction, pos)
}

fn _draw_map(map:&MirrorMap, beams:&Vec<LightBeam>) {
    println!("");
    println!("");
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            let mut beam_char = map[y][x];
            for beam in beams {
                let (dir, (bx,by)) = beam;
                if x == *bx && y == *by {
                    beam_char = match dir {
                        BeamDirection::North => '^',
                        BeamDirection::East => '>',
                        BeamDirection::West => '<',
                        BeamDirection::South => 'v',
                    }
                }
            }
            print!("{}", beam_char);
        }
        print!("\n");
    }
    std::thread::sleep(std::time::Duration::from_millis(500));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_energized_tiles() {
        assert_eq!(46, count_energized_tiles(".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|...."));
    }

    #[test]
    fn test_count_max_energized_tiles() {
        assert_eq!(46, count_energized_tiles(".|...\\....\n|.-.\\.....\n.....|-...\n........|.\n..........\n.........\\\n..../.\\\\..\n.-.-/..|..\n.|....-|.\\\n..//.|...."));
    }
}