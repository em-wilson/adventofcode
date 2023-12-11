pub fn sum_shortest_paths(input:&str, expansion_factor:usize) -> usize {
    let space = input.split("\n")
        .map(|line|line.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let expansions = expand_space(&space);
    let stars = collect_stars(&space, &expansions, expansion_factor);
    let distances = map_distances(&stars);
    distances.into_iter().sum()
}

fn map_distances(input:&Vec<(usize,usize)>) -> Vec<usize> {
    input.into_iter().enumerate().skip(1)
        .flat_map(|(i,pos1)|input.into_iter().take(i).map(|pos2|shortest_path(*pos1,*pos2)))
        .collect()
}

fn collect_stars(input:&Vec<Vec<char>>, expansions:&Vec<Vec<usize>>, expansion_factor: usize) -> Vec<(usize,usize)> {
    let mut stars:Vec<(usize,usize)> = Vec::new();
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] != '.' {
                let xpand_x_count = expansions[0].iter().filter(|pos|*pos < &x).count();
                let xpand_y_count = expansions[1].iter().filter(|pos|*pos < &y).count();
                let x_pos = x + (xpand_x_count * expansion_factor) - xpand_x_count;
                let y_pos = y + (xpand_y_count * expansion_factor) - xpand_y_count;
                stars.push((x_pos,y_pos));
            }
        }
    }
    stars
}

fn expand_space(input:&Vec<Vec<char>>) -> Vec<Vec<usize>> {
     let x_expansions = (0..input[0].len()).into_iter()
        .filter(|x|input.iter().map(|l|l[*x]).all(|c|c==input[0][*x]))
        .collect();

    let y_expansions = input.into_iter().enumerate()
        .filter(|(_,line)|line.into_iter().all(|c|*c == line[0]))
        .map(|(i,_)|i)
        .collect();

    vec!(x_expansions, y_expansions)
}

fn shortest_path(a:(usize,usize), b:(usize,usize)) -> usize {
    let (x1, y1) = a;
    let (x2, y2) = b;

    (std::cmp::max(x1, x2) - std::cmp::min(x1, x2))
    +
    (std::cmp::max(y1, y2) - std::cmp::min(y1, y2))
}

#[cfg(test)]
mod tests {
    use super::*;

    // OP = √((x2 – x1)² + (y2 – y1)²)
    //  √(4-0)² + (2 – 0)²)
    // √(16 + 4)
    // = 5

    // 4,0 -> 0,2 == 4 + 2 = 6

//1,6 ... 5,11  = 4 + 5 = 9
    // OP = √((5 – 1)² + (11 – 6)²)
     // OP = √((16 + 25)
     // OP = 5

     #[test]
     fn test_shortest_path() {
        assert_eq!(6, shortest_path((4,0), (0,2)));
        assert_eq!(9, shortest_path((1,6), (5,11)));
     }


    #[test]
    fn test_sum_shortest_paths() {
        assert_eq!(374, sum_shortest_paths("...#......\n.......#..\n#.........\n..........\n......#...\n.#........\n.........#\n..........\n.......#..\n#...#.....", 2));
    }
}