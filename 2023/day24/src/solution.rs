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

fn stones_intersect(h1: &HailStone, h2: &HailStone, min:f64, max:f64) -> bool {
    println!("checking");
    if let Some(intersection) = get_intersection_2d(h1, h2) {
        println!("Got an intersection {:?}", intersection);
        let (x, y) = intersection;
            
        if (x > min && x < max) && (y > min && y < max) {
            return true;
        }
    }
    false
}

fn get_intersection_2d(h1: &HailStone, h2: &HailStone) -> Option<(f64, f64)> {
    let determinant = (h1.1.0 * h2.1.1 - h1.1.1 * h2.1.0) as f64;

    if determinant == 0.0 {
        return None;
    }
    let time: f64 = ((h2.0.0 - h1.0.0) * h2.1.1 - (h2.0.1 - h1.0.1) * h2.1.0) as f64 / determinant;
    let time2: f64 = ((h1.0.0 - h2.0.0) * h1.1.1 - (h1.0.1 - h2.0.1) * h1.1.0) as f64 / determinant;

    if time < 0.0 {
        return None;
    }
    if time * time2 > 0.0 {
        return None;
    }
    let x = h1.0.0 as f64 + h1.1.0 as f64* time;
    let y = h1.0.1 as f64 + h1.1.1 as f64 *time;
    
    Some((x, y))
}


// fn stones_intersect(a:&HailStone, b:&HailStone, min:f64, max:f64) -> bool {
//     let ((a_x1, a_y1, _), (a_vx, a_vy, _)) = a;
//     let (a_x2, a_y2) = (a_x1+a_vx, a_y1+a_vy);
//     let ((b_x1, b_y1, _), (b_vx, b_vy, _)) = b;
//     let (b_x2, b_y2) = (b_x1+b_vx, b_y1+b_vy);

//     let line1 = Line::<f64>::from([(a_x1.clone(),a_y1.clone()),(a_x2.clone(),a_y2.clone())]);
//     let line2 = Line::<f64>::from([(b_x1.clone(),b_y1.clone()),(b_x2.clone(),b_y2.clone())]);

//     if let Some(Intersection::Intersection(intersection)) = intersect(&line1, &line2) {
//         println!("Intersection {:?}", intersection);
//         return true;
//     } else {
//         return false;
//     }

    // let a_slope = (a_y2 - a_y1) / (a_x2 - a_x1);
    // let b_slope = (b_y2 - b_y1) / (b_x2 - b_x1);

    // let a_y_intersection = a_y1 - (a_slope * a_x1);
    // let b_y_intersection = b_y1 - (b_slope * b_x1);

    // let (x_intersect, y_intersect) = find_intersection((a_slope, a_y_intersection), (b_slope, b_y_intersection));

    // println!("{:?} {:?}", x_intersect, y_intersect);

    // x_intersect >= min && x_intersect <= max && y_intersect >= min && y_intersect <= max
// }

// fn find_intersection(left:(f64, f64),right:(f64,f64)) -> (f64, f64) {
//     let (a,b) = left;
//     let (c,d) = right;
//     let x_intersect = (d - c)/(a - b);
//     let y_intersect = (a * ((d - c)/(a-b))) + c;
//     (x_intersect, y_intersect)
// }

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