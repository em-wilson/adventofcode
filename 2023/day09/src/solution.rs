fn next_in_sequence(sequence:Vec<isize>) -> isize {
    let l = sequence.last().unwrap();
    let diff_seq:Vec<_> = sequence.windows(2).map(|w|w[1] as isize - w[0] as isize).collect();
    let f = diff_seq.first().unwrap();

    if diff_seq.iter().all(|v|v == f) {
        return f+l;
    }

    let next = next_in_sequence(diff_seq);
    return l + next;
}

pub fn sum_of_next_sequences(input:&str) -> isize {
    input.split("\n")
        .map(|line|line.split(" "))
        .map(|s|s.map(|t|t.parse::<isize>().unwrap()))
        .map(|s|next_in_sequence(s.collect()))
        .sum()
}

fn prev_in_sequence(sequence:Vec<isize>) -> isize {
    let f = sequence.first().unwrap();
    let diff_seq:Vec<_> = sequence.windows(2).map(|w|w[1] as isize - w[0] as isize).collect();
    let l = diff_seq.last().unwrap();

    if diff_seq.iter().all(|v|v == l) {
        return f-l;
    }

    let prev = prev_in_sequence(diff_seq);
    return f - prev;
}

pub fn sum_of_prev_sequences(input:&str) -> isize {
    input.split("\n")
        .map(|line|line.split(" "))
        .map(|s|s.map(|t|t.parse::<isize>().unwrap()))
        .map(|s|prev_in_sequence(s.collect()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sum_of_next_sequences() {
        let seq = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        assert_eq!(114, sum_of_next_sequences(seq));
    }

    #[test]
    fn test_next_in_sequence() {
        assert_eq!(18, next_in_sequence(vec!(0,3,6,9,12,15)));
        assert_eq!(28, next_in_sequence(vec!(1,3,6,10,15,21)));
        assert_eq!(68, next_in_sequence(vec!(10,13,16,21,30,45)));
    }

    #[test]
    fn test_sum_of_prev_sequences() {
        let seq = "0 3 6 9 12 15\n1 3 6 10 15 21\n10 13 16 21 30 45";
        assert_eq!(2, sum_of_prev_sequences(seq));
    }

    #[test]
    fn test_prev_in_sequence() {
        assert_eq!(-3, prev_in_sequence(vec!(0,3,6,9,12,15)));
        assert_eq!(0, prev_in_sequence(vec!(1,3,6,10,15,21)));
        assert_eq!(5, prev_in_sequence(vec!(10,13,16,21,30,45)));
    }
}