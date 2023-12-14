pub fn summarize_smudges(input:&str) -> usize {
    summarize(input, find_smudge)
}

pub fn summarize_counts(input:&str) -> usize {
    summarize(input, find_reflection)
}

fn summarize(input:&str, process_fn:impl Fn(&str) -> (usize, usize)) -> usize {
    input.split("\n\n")
        .map(|input|{
            let result = process_fn(input);
            result.0 * 100 + result.1
        })
        .sum()
}

fn process_mirrors(input:&str, scan_cols:impl Fn(&Vec<Vec<char>>) -> Option<usize>) -> (usize, usize) {
    let map_rows:Vec<Vec<char>> = input.split("\n")
        .map(|line|line.chars().collect())
        .collect();

    let map_cols:Vec<Vec<char>> = (0..map_rows[0].len())
        .map(|i|map_rows.iter().map(|r|r[i]).collect())
        .collect();

    if let Some(result) = scan_cols(&map_rows) {
        return (result, 0);
    }

    if let Some(result) = scan_cols(&map_cols) {
        return (0, result);
    }

    panic!("{}\n\nno reflection found!", input);
}

fn find_reflection(input:&str) -> (usize, usize) {
    process_mirrors(input, |lines| {
        let column_length = lines.len();
        for x in 0..(column_length - 1) {
            let peek_len = std::cmp::max(x, column_length+1-x);
            for peek in 1..peek_len {
                let lines_match = lines[x+1-peek] == lines[x+peek];

                if lines_match && (x + 1 - peek == 0 || x + peek == column_length-1) {
                    return Some(x + 1);
                }

                if lines[x+1-peek] != lines[x+peek] {
                    break;
                }
            }
        }
        return None;
    })
}

fn find_smudge(input:&str) -> (usize, usize) {
    process_mirrors(input, |lines| {
        let column_length = lines.len();
        for x in 0..(column_length - 1) {
            let mut differences = 0;
            let peek_len = std::cmp::max(x, column_length+1-x);
            for peek in 1..peek_len {
                let x1 = x+1-peek;
                let x2 = x+peek;
                let row1 = &lines[x+1-peek];
                let row2 = &lines[x+peek];
                for i in 0..row1.len() {
                    if row1[i] != row2[i] {
                        differences += 1;
                    }
                }
                if x1 == 0 || x2 == column_length - 1 {
                    break;
                }
            }

            if differences == 1 {
                return Some(x + 1);
            }
        }
        return None;
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_summarize() {
        let input = fs::read_to_string("test_input.txt")
            .expect("Could not read file test_input.txt");
        
        assert_eq!(405, summarize_counts(&input));
        assert_eq!(400, summarize_smudges(&input));
    }

    #[test]
    fn test_find_smudge() {
        assert_eq!((3, 0), find_smudge("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#."));
        assert_eq!((1, 0), find_smudge("#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"));
    }

    #[test]
    fn test_find_reflection() {
        assert_eq!((0, 5), find_reflection("#.##..##.\n..#.##.#.\n##......#\n##......#\n..#.##.#.\n..##..##.\n#.#.##.#."));
        assert_eq!((4, 0), find_reflection("#...##..#\n#....#..#\n..##..###\n#####.##.\n#####.##.\n..##..###\n#....#..#"));
    }
}