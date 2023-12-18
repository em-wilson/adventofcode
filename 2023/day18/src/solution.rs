use std::collections::HashSet;

pub enum LineParser {
    Straight,
    Hex
}

impl LineParser {
    fn parse(&self, line:&str) -> (String, usize) {
        match self {
            LineParser::Straight => {
                let parts:Vec<_> = line.split(" ").collect();
                let direction = parts[0];
                let steps = parts[1].parse::<usize>().unwrap();
                (direction.to_string(), steps)        
            },
            LineParser::Hex => convert_dig_code(line)
        }
    }
}

pub fn count_dugout_size(input:&str, line_parser:LineParser) -> usize {
    let mut dug_positions = HashSet::new();
    let mut pos:(isize, isize) = (0,0);
    dug_positions.insert(pos);
    for line in input.split("\n") {
        let (direction, steps) = line_parser.parse(&line);
        for _ in 1..=steps {
            let (x,y) = pos;
            pos = match direction.as_str() {
                "R" => (x + 1, y),
                "L" => (x - 1, y),
                "U" => (x, y - 1),
                "D" => (x, y + 1),
                _   => panic!("unknown direction {}", direction)
            };
            dug_positions.insert(pos);
        }
    }
    let corrected_dug_positions = tidy_dug_positions(&dug_positions);
    let (max_x, max_y) = get_bounds(&corrected_dug_positions);
    let grid_area = (max_x + 1) * (max_y + 1);
    grid_area - count_open_spaces(&corrected_dug_positions)
}

// Snap to grid with origin 0,0 ; removes negative numbers
fn tidy_dug_positions(positions:&HashSet<(isize, isize)>) -> HashSet<(usize, usize)> {
    let (mut x_delta, mut y_delta) = positions.iter()
        .fold((0,0), |(x_old, y_old),(x,y)|(std::cmp::min(x_old, *x),std::cmp::min(y_old,*y)));

    if x_delta < 0 {
        x_delta = x_delta.abs();
    } else {
        x_delta = 0;
    }

    if y_delta < 0 {
        y_delta = y_delta.abs();
    } else {
        y_delta = 0;
    }

    positions.iter()
        .map(|(x,y)|((x + x_delta) as usize, (y + y_delta) as usize))
        .collect()
}

fn get_bounds(positions:&HashSet<(usize, usize)>) -> (usize, usize) {
    positions.iter()
        .fold((0,0), |(x_old, y_old),(x,y)|(std::cmp::max(x_old, *x),std::cmp::max(y_old,*y)))
}

fn count_open_spaces(dug_positions:&HashSet<(usize, usize)>) -> usize {
    let (max_x, max_y) = get_bounds(dug_positions);

    let mut map = vec![vec!['.'; max_x + 1]; max_y + 1];
    let mut frontier = Vec::new();
    let mut open_spaces = HashSet::new();
    for y in 0..=max_y {
        for x in 0..=max_x {
            if dug_positions.contains(&(x,y)) {
                map[y][x] = '#';
            } else if x == 0 || x == max_x || y == 0 || y == max_y {
                frontier.push((x,y));
            }
        }
    }

    while let Some(coord) = frontier.pop() {
        let (x,y) = coord;
        if map[y][x] == '#' {
            continue;
        }
        if open_spaces.insert(coord) {
            if x > 0 { frontier.push((x - 1, y)); }
            if y > 0 { frontier.push((x, y - 1)); }
            if x < max_x { frontier.push((x + 1, y)); }
            if y < max_y { frontier.push((x, y + 1)); }
        }
    }
    open_spaces.len()
}

fn convert_dig_code(code:&str) -> (String, usize) {
    let code_chars:Vec<char> = code.chars().collect();
    let digit_s:String = code_chars.clone().iter().skip(2).take(5).collect();
    let digit = usize::from_str_radix(&digit_s, 16).unwrap();
    let direction = match code_chars[7] {
        '0' => "R",
        '1' => "D",
        '2' => "L",
        '3' => "U",
         _  => todo!()
    };
    return (direction.to_string(), digit);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_count_dugout_size() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(62, count_dugout_size(&input, LineParser::Straight))
    }

    #[test]
    fn test_count_dugout_size() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(952408144115, count_dugout_size(&input, LineParser::Hex))
    }

    #[test]
    fn test_convert_dig_code() {
        assert_eq!(("R".to_string(), 461937), convert_dig_code("(#70c710)"));
        assert_eq!(("D".to_string(), 56407), convert_dig_code("(#0dc571)"));
        assert_eq!(("R".to_string(), 356671), convert_dig_code("(#5713f0)"));
        assert_eq!(("D".to_string(), 863240), convert_dig_code("(#d2c081)"));
        assert_eq!(("R".to_string(), 367720), convert_dig_code("(#59c680)"));
        assert_eq!(("D".to_string(), 266681), convert_dig_code("(#411b91)"));
        assert_eq!(("L".to_string(), 577262), convert_dig_code("(#8ceee2)"));
    }
}