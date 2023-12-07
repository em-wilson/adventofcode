mod solution;

fn main() {
    // (time, distance)
    let races = vec!(
        (58, 434),
        (81, 1041),
        (96, 2219),
        (76, 1218)
    );
    println!("Results for A: {}", solution::multiply_possible_hold_time(races));

    println!("Results for B: {}", solution::multiply_possible_hold_time(vec!((58819676, 434104122191218))));
}