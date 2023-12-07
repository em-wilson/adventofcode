mod solution;

fn main() {
    // (time, distance)
    let races = vec!(
        (58, 434),
        (81, 1041),
        (96, 2219),
        (76, 1218)
    );
    println!("Results for A: {}", day06::run_a(""));

    println!("Results for B: {}", day06::run_b(""));
}