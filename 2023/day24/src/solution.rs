pub fn count_intersections(input:&str, min:f64, max:f64) -> usize {
    let hailstones:Vec<HailStone> = input.split("\n")
        .map(parse_hailstone)
        .collect();

    let mut count = 0;
    for x in 0..hailstones.len() {
        for y in x+1..hailstones.len() {
            if stones_intersect(&hailstones[x], &hailstones[y], min, max) {
                count += 1;
            }
        }
    }
    count
}

type HailStone = (Position, Velocity);
type Position = (f64, f64, f64);
type Velocity = (f64, f64, f64);

fn stones_intersect(a:&HailStone, b:&HailStone, min:f64, max:f64) -> bool {
    let ((a_x1, a_y1, _), (a_vx, a_vy, _)) = a;
    let (a_x2, a_y2) = (a_x1+a_vx, a_y1+a_vy);
    let ((b_x1, b_y1, _), (b_vx, b_vy, _)) = b;
    let (b_x2, b_y2) = (b_x1+b_vx, b_y1+b_vy);

    let a_x12 = a_x1 - a_x2;
    let b_x12 = b_x1 - b_x2;
    let a_y12 = a_y1 - a_y2;
    let b_y12 = b_y1 - b_y2;

    let c = a_x12 * b_y12 - a_y12 * b_x12;

    if c.abs() > 0_f64 {
        let a = a_x1 * a_y2 - a_y1 * a_y2;
        let b = b_x1 * b_y2 - b_y1 * b_x2;

        let x = (a * b_x12 - b * a_x12) / c;
        let y = (a * b_y12 - b * a_y12) / c;

        if x >= min && x <= max && y >= min && y <= max {
            return true;
        }
    }

    false
}

fn parse_hailstone(input:&str) -> HailStone {
    let parts:Vec<_> = input.split(" @ ").collect();
    let position:Vec<f64> = parts[0].split(", ")
        .map(|p|p.parse::<f64>().unwrap())
        .collect();
    let velocity:Vec<f64> = parts[0].split(", ")
        .map(|p|p.parse::<f64>().unwrap())
        .collect();

    ((position[0], position[1], position[2]), (velocity[0], velocity[1], velocity[2]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_count_intersections() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");

        assert_eq!(2, count_intersections(&input, 7_f64, 27_f64));
    }
}