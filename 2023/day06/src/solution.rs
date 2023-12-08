use roots::find_roots_quadratic;

pub fn possible_hold_time(race_time:u64, distance_record:u64) -> u64 {
    let roots = get_roots(race_time, distance_record);
    let (min_bound, max_bound) = roots;
    // let result = 1+roots.1-roots.0;
    return max_bound+1-min_bound;
}

pub fn multiply_possible_hold_time(races: Vec<(u64, u64)>) -> u64 {
    return races.into_iter().map(|(race_time, distance_record)| possible_hold_time(race_time, distance_record))
        .product();
}

fn get_roots(race_time:u64, distance_record:u64) -> (u64, u64) {
    //x = 0..race_time; (hold time)
    //y = (race_time-x)*x  (distance)

    // two variables quad: x2-4xy+3y2=0
    // y=ax-x^2
    // y == distance travelled
    // x == time held
    // a == race_time
    let roots_binding = find_roots_quadratic(-1 as f64, race_time as f64, -1 as f64 * distance_record as f64);
    let roots = roots_binding.as_ref();
    let mut min_bound = roots[0].ceil() as u64;
    let mut max_bound = roots[1].floor() as u64;
    if roots[0].fract() == 0.0 {
        min_bound += 1;
    }
    if roots[1].fract() == 0.0 {
        max_bound -= 1;
    }
    return (min_bound, max_bound);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_possible_hold_times() {
        assert_eq!(4, possible_hold_time(7, 9));
        assert_eq!(8, possible_hold_time(15, 40));
        assert_eq!(9, possible_hold_time(30, 200));
    }

    #[test]
    fn test_get_root() {
        assert_eq!((2, 5), get_roots(7,9));
        assert_eq!((11, 19), get_roots(30, 200));
    }

    #[test]
    fn test_multiply_possible_hold_times() {
        let races = vec!((7,9), (15, 40), (30,200));
        assert_eq!(288, multiply_possible_hold_time(races));
    }

    #[test]
    fn test_multiply_possible_hold_times_single_race() {
        let races = vec!((71530, 940200));
        assert_eq!(71503, multiply_possible_hold_time(races));
    }
}