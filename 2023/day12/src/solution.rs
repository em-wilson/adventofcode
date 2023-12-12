use rayon::prelude::*;
use std::collections::HashMap;

pub fn calculate_combinations(input:&str, folds:usize) -> usize {
    let lines:Vec<(Vec<char>, Vec<usize>)> = input.split("\n")
        .map(|line|unfold_line(line, folds))
        .map(|line|{
            let parts:Vec<&str> = line.split(" ").collect();
            let data:Vec<char> = parts[0].chars().collect();
            let groups:Vec<usize> = parts[1].split(",").map(|n|n.parse().unwrap()).collect();
            (data, groups)
        })
        .collect();

    lines.par_iter()
        .map(|(data, groups)|{
            let mut memoization: HashMap<(Vec<usize>, Vec<char>), usize> = HashMap::new();
            calculate_solutions(data, groups, &mut memoization)
        })
        .sum()
}

fn calculate_solutions(data:&Vec<char>, groups:&Vec<usize>, memoization: &mut HashMap<(Vec<usize>, Vec<char>), usize>) -> usize {
    if data.is_empty() {
        if groups.is_empty() {
            return 1;
        } else {
            return 0;
        }
    }

    match data[0] {
        '.' => calculate_solutions(&data[1..].to_vec(), groups, memoization),
        '#' => calculate_hash_solutions(data, groups, memoization),
        '?' => calculate_solutions(&data[1..].to_vec(), groups, memoization) + calculate_hash_solutions(data, groups, memoization),
        _ => panic!(">.> WHAT DID YOU DO?"),
    }
}

fn calculate_hash_solutions(data:&Vec<char>, groups:&Vec<usize>, memoization: &mut HashMap<(Vec<usize>, Vec<char>), usize>) -> usize {
    // Anything smaller than 2 isn't worth looking up in the memo
    const MIN_YIELD:usize = 2;
    if data.len() > MIN_YIELD {
        if let Some(&result) = memoization.get(&(groups.clone(), data.clone())) {
            return result;
        }
    }

    if groups.is_empty() {
        return 0;
    }

    let x = groups[0] as usize;
    if data.len() < x {
        return 0;
    }
    for i in 0..x {
        if data[i] == '.' {
            return 0;
        }
    }
    if data.len() == x {
        if groups.len() == 1 {
            return 1;
        }
        return 0;
    }
    if data[x] == '#' {
        return 0;
    }
    let result = calculate_solutions(&data[(x + 1)..].to_vec(), &groups[1..].to_vec(), memoization);
    if data.len() > MIN_YIELD {
        memoization.insert((groups.clone(), data.clone()), result);
    }
    result
}

fn unfold_line(input:&str, folds:usize) -> String {
    if folds == 1 {
        return input.to_string();
    }
    let parts:Vec<&str> = input.split(" ").collect();
    let springs = vec![parts[0]; folds].join("?");
    let conditions = vec![parts[1]; folds].join(",");
    springs + " " + &conditions
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_combinations_single_fold() {
        assert_eq!(1, calculate_combinations("???.### 1,1,3", 1));
        assert_eq!(4, calculate_combinations(".??..??...?##. 1,1,3", 1));
        assert_eq!(1, calculate_combinations("?#?#?#?#?#?#?#? 1,3,1,6", 1));
        assert_eq!(1, calculate_combinations("????.#...#... 4,1,1", 1));
        assert_eq!(4, calculate_combinations("????.######..#####. 1,6,5", 1));
        assert_eq!(10, calculate_combinations("?###???????? 3,2,1", 1));
    }

    #[test]
    fn test_calculate_combinations_five_folds() {
        assert_eq!(1, calculate_combinations("???.### 1,1,3", 5));
        assert_eq!(16384, calculate_combinations(".??..??...?##. 1,1,3", 5));
        assert_eq!(1, calculate_combinations("?#?#?#?#?#?#?#? 1,3,1,6", 5));
        assert_eq!(16, calculate_combinations("????.#...#... 4,1,1", 5));
        assert_eq!(2500, calculate_combinations("????.######..#####. 1,6,5", 5));
        assert_eq!(506250, calculate_combinations("?###???????? 3,2,1", 5));
    }

    #[test]
    fn test_unfold_line() {
        assert_eq!(".#?.#?.#?.#?.# 1,1,1,1,1", unfold_line(".# 1", 5));
        assert_eq!("???.###????.###????.###????.###????.### 1,1,3,1,1,3,1,1,3,1,1,3,1,1,3", unfold_line("???.### 1,1,3", 5));
    }
}