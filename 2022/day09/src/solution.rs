use std::collections::HashSet;

pub fn count_tail_positions(input:&str, knots:usize) -> usize {
    let mut tail_pos:Vec<(isize,isize)> = vec![(0,0); knots];
    let mut head_pos:(isize,isize) = (0,0);
    let mut tail_visited:HashSet<(isize,isize)> = HashSet::new();
    // let mut cur_move = 0;

    for line in input.split("\n") {
        let parts = line.split(" ").collect::<Vec<_>>();
        let (dir_x, dir_y) = match parts[0] {
            "R" => (1, 0),
            "U" => (0, -1),
            "L" => (-1, 0),
            "D" => (0, 1),
            _   => todo!(),
        };
        let spaces = parts[1].parse::<usize>().unwrap();
        for _ in 1..=spaces {
            let (mut head_x, mut head_y) = head_pos;
            head_x += dir_x;
            head_y += dir_y;
            head_pos = (head_x, head_y);

            tail_pos[0] = move_tail(head_pos, tail_pos[0]);

            for i in 1..tail_pos.len() {
                tail_pos[i] = move_tail(tail_pos[i-1], tail_pos[i]);
            }

            // cur_move += 1;
            // print!("{}: {:?}", cur_move, head_pos);
            // tail_pos.iter().for_each(|pos|print!(" {:?}", pos));
            // println!("");
            
            // tail_pos.iter().for_each(|pos|{tail_visited.insert(*pos);});
            tail_visited.insert(*tail_pos.last().unwrap());
        }
    }
    tail_visited.len()
}

fn move_tail(head_pos:(isize, isize), tail_pos:(isize, isize)) -> (isize, isize) {
    let (mut tail_x, mut tail_y) = tail_pos;
    let (head_x, head_y) = head_pos;
    let (mut dir_x, mut dir_y) = (0,0);
    if head_x < tail_x {
        dir_x = -1;
    }
    if head_x > tail_x {
        dir_x = 1;
    }
    if head_y < tail_y {
        dir_y = - 1;
    }
    if head_y > tail_y {
        dir_y = 1;
    }

    while (head_x-tail_x).abs() > 1 || (head_y-tail_y).abs() > 1  {
        tail_x += dir_x;
        tail_y += dir_y;
    }
    (tail_x, tail_y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_tail_positions() {
        assert_eq!(13, count_tail_positions("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2", 1));
        // assert_eq!(1, count_tail_positions("R 4\nU 4\nL 3\nD 1\nR 4\nD 1\nL 5\nR 2", 9));
        // assert_eq!(13, count_tail_positions("R 5\nU 8\nL 8\nD 3\nR 17\nD 10\nL 25\nU 20", 9));
    }
}