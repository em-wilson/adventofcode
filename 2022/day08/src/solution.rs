pub fn count_visible_trees(input:&str) -> usize {
    let grid:Vec<Vec<u32>> = input.split("\n")
        .map(|line|line.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let length = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut findings:Vec<Option<bool>> = vec![None; (length * width) as usize];
    
    for x in 0..length {
        let x = x as i32;
        scan( &grid, &mut findings, x,         0,          0,   1 ); // Scan down
        scan( &grid, &mut findings, x,         width-1,    0,  -1 ); // Scan up
    }
    for y in 0..width {
        let y = y as i32;
        scan( &grid, &mut findings, 0,         y,          1,  0 ); // Scan right
        scan( &grid, &mut findings, length-1,  y,         -1,  0 ); // Scan left
    }

    
    findings.into_iter().filter(|f|{
        if let Some(result) = f {
            return *result;
        }
        false
    }).collect::<Vec<_>>().len()
}

fn scan( grid:&Vec<Vec<u32>>, findings:&mut Vec<Option<bool>>, x:i32, y:i32, x_delta:i32, y_delta: i32 ) {
    let length = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut new_x = x;
    let mut new_y = y;
    let mut index = (y * length) + x;

    findings[index as usize] = Some(true);
    let mut max_height = -1;

    while new_x >= 0 && new_x < length && new_y >= 0 && new_y < width {
        index = (new_y * length) + new_x;
        let new_height = grid[new_x as usize][new_y as usize] as i32;
        if new_height > max_height {
            max_height = new_height;
            if index == 7 {
            }
            findings[index as usize] = Some(true);
        }
        new_x += x_delta;
        new_y += y_delta;
    }
}

fn score( grid:&Vec<Vec<u32>>, x:i32, y:i32, x_delta:i32, y_delta: i32 ) -> usize {
    let length = grid.len() as i32;
    let width = grid[0].len() as i32;

    let max_height = grid[x as usize][y as usize];
    let mut tree_count = 0;

    let mut new_x = x + x_delta;
    let mut new_y = y + y_delta;
    while new_x >= 0 && new_x < length && new_y >= 0 && new_y < width {
        tree_count += 1;
        if grid[new_x as usize][new_y as usize] >= max_height {
            break;
        }
        new_x += x_delta;
        new_y += y_delta;
    }

    tree_count
}

pub fn calculate_max_scenic_score(input:&str) -> usize {
    let grid:Vec<Vec<u32>> = input.split("\n")
        .map(|line|line.chars().map(|c|c.to_digit(10).unwrap()).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let length = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut max_score = 0;

    for x in 0..length {
        for y in 0..width {
            let x = x as i32;
            let y = y as i32;
            let scenic_score = 
                score( &grid, x, y,  0,  1 ) // Scan down
              * score( &grid, x, y,  0, -1 ) // Scan up
              * score( &grid, x, y,  1,  0 )  // Scan right
              * score( &grid, x, y, -1,  0 ); // Scan left
            max_score = std::cmp::max(max_score, scenic_score);
        }
    }
    
    max_score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_visible_trees() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(21, count_visible_trees(input));
    }

    #[test]
    fn test_calculate_max_scenic_score() {
        let input = "30373\n25512\n65332\n33549\n35390";
        assert_eq!(8, calculate_max_scenic_score(input));
    }
}