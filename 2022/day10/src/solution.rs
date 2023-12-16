pub fn sum_during_cycles(input:&str, interesting_cycles:&Vec<i32>) -> i32 {
    let mut x_register:i32 = 1;
    let mut cycle = 0;
    input.split("\n")
        .flat_map(|line|{
            let parts:Vec<&str> = line.split(" ").collect();
            let mut result = vec!();
            let (cycles, register_change) = match parts[0] {
                "noop" => (1, 0),
                "addx" => (2, parts[1].parse::<i32>().unwrap()),
                _      => todo!()
            };

            for _ in 0..cycles {
                cycle += 1;
                if interesting_cycles.contains(&cycle) {
                    result.push(x_register * cycle);
                }
            }
            x_register += register_change;
            result
        })
        .sum::<i32>()
}

pub fn render_image(input:&str) -> String {
    let mut x_register:i32 = 1;
    let mut pos = 0;
    let result =input.split("\n")
        .flat_map(|line|{
            let parts:Vec<&str> = line.split(" ").collect();
            let mut result = vec!();
            let (cycles, register_change) = match parts[0] {
                "noop" => (1, 0),
                "addx" => (2, parts[1].parse::<i32>().unwrap()),
                _      => todo!()
            };

            for _ in 0..cycles {
                if x_register >= pos - 1 && x_register <= pos + 1 {
                    result.push('#');
                } else {
                    result.push('.');
                }
                pos += 1;
                if pos == 40 {
                    pos = 0;
                }
            }
            x_register += register_change;
            result
        })
        .collect::<Vec<_>>()
        .chunks(40)
        .map(|c|c.iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");

    result
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_short_loop() {
        let input = "noop\naddx 3\naddx -5";
        assert_eq!(1, sum_during_cycles(input, &vec!(1)));
        assert_eq!(2, sum_during_cycles(input, &vec!(2)));
        assert_eq!(3, sum_during_cycles(input, &vec!(3)));
        assert_eq!(16, sum_during_cycles(input, &vec!(4)));
        assert_eq!(20, sum_during_cycles(input, &vec!(5)));
    }

    #[test]
    fn test_test_loop() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(13140, sum_during_cycles(&input, &vec!(20,60,100,140,180,220)));
    }

    #[test]
    fn test_render_image() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        let expected = "##..##..##..##..##..##..##..##..##..##..\n###...###...###...###...###...###...###.\n####....####....####....####....####....\n#####.....#####.....#####.....#####.....\n######......######......######......####\n#######.......#######.......#######.....";
        assert_eq!(expected, render_image(&input));
    }
}