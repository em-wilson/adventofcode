pub fn possible_hold_time(race_time:u64, distance_record:u64) -> u64 {
    let mut times = Vec::new();
    for hold_time in 0..=race_time {
        // s+(t-s)(s);
        let travel_time = race_time-hold_time;
        let d =travel_time * hold_time;

        if d > distance_record {
            times.push(hold_time);
        }
    }
    return times.len() as u64;
}

pub fn multiply_possible_hold_time(races: Vec<(u64, u64)>) -> u64 {
    return races.into_iter().map(|(race_time, distance_record)| possible_hold_time(race_time, distance_record))
        .product();
}

// fn get_root(race_time:u64, distance_record:u64) -> u64 {
//     //x = 0..race_time; (hold time)
//     //y = (race_time-x)*x  (distance)

//     // two variables quad: x2-4xy+3y2=0
// }

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