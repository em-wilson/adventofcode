pub enum LineParser {
    Straight,
    Hex
}

impl LineParser {
    fn parse(&self, line:&str) -> (String, usize) {
        let parts:Vec<_> = line.split(" ").collect();
        match self {
            LineParser::Straight => {
                let direction = parts[0];
                let steps = parts[1].parse::<usize>().unwrap();
                (direction.to_string(), steps)        
            },
            LineParser::Hex => convert_dig_code(parts[2])
        }
    }
}

pub fn count_dugout_size(input:&str, line_parser:LineParser) -> usize {
    let mut pos:(isize, isize) = (0,0);
    let mut lace_positions = Vec::new();
    lace_positions.push(pos);
    let mut perimeter = 2;
    for line in input.split("\n") {
        let (direction, steps) = line_parser.parse(&line);
        let (x,y) = pos;
        pos = match direction.as_str() {
            "R" => (x + steps as isize, y),
            "L" => (x - steps as isize, y),
            "U" => (x, y - steps as isize),
            "D" => (x, y + steps as isize),
            _   => panic!("unknown direction {}", direction)
        };
        perimeter += steps;
        lace_positions.push(pos);
    }

    let area:isize = lace_positions.windows(2).map(|window|{
        let (x1,y1) = window[0];
        let (x2,y2) = window[1];
        (x1 as isize * y2 as isize) - (y1 as isize * x2 as isize)
    }).sum();
    ((area + perimeter as isize) / 2) as usize
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
    fn test_count_massive_dugout_size() {
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